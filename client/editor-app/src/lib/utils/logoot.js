// paper: https://inria.hal.science/inria-00432368/document
// reference implementations:
// https://github.com/usecanvas/logoot-js/blob/master/lib/logoot/sequence.js
// https://github.com/ravern/logoot/blob/master/doc/doc.go


/**
 * The Identifier is a pair: <pos, siteId>
 * where pos is an integer and siteId is
 * is a global id for a site; a site is an
 * independent instance of a Logoot crdt.
 */
class Identifier {
    /**
     * @param {number} pos - Integer
     * @param {number} siteId - Id of corresponding site 
     */
    constructor(pos, siteId) {
        this.pos = pos;
        this.siteId = siteId;
        
    }

    /**
     * @param {{ pos: number; siteId: number; }} other
     */
    compareTo(other) {
        // Compare first by position
        if (this.pos < other.pos) return -1;
        if (this.pos > other.pos) return 1;
        
        // If positions are the same, compare by siteId
        if (this.siteId < other.siteId) return -1;
        if (this.siteId > other.siteId) return 1;
        
        return 0; // they are equal
    }

    toString() {
        return `<${this.pos},${this.siteId}>`;
    }
}

/**
 * The PositionIdentifier is a pair: <pos, hs>
 * where pos is a position, which is a list of identifiers
 * and hs is a clock value.
 */
class PositionIdentifier {
    /**
     * @param {Identifier[]} identifiers - An array of Identifier objects.
     * @param {number} clock - A clock value, typically representing a logical or physical timestamp.
     */
    constructor(identifiers, clock) {
        this.identifiers = identifiers;
        this.clock = clock;
    }

    /**
     * Compares this PositionIdentifier with another to determine order.
     * @param {PositionIdentifier} other - The other PositionIdentifier to compare to.
     * @returns {number} - -1 if this is less than other, 1 if this is greater than other, 0 if they are equal.
     */
    compareTo(other) {
        // edge cases
        if (this.identifiers.length === 0 && other.identifiers.length === 0) return 0;
        if (this.identifiers.length === 0) return -1;
        if (other.identifiers.length === 0) return 1;

        // compare each identifier
        const minLength = Math.min(this.identifiers.length, other.identifiers.length);
        for (let i = 0; i < minLength; i++) {
            const result = this.identifiers[i].compareTo(other.identifiers[i]);
            if (result != 0) return result;
        }

        // if all are equal, consider the length of the identifier lists
        if (this.identifiers.length < other.identifiers.length) return -1;
        if (this.identifiers.length > other.identifiers.length) return 1;

        // if still equal, return 0 - clock values are not considered: 
        // "We only compare positions – and not logical clocks –
        // since there can not be, in the same model, two lines with
        // the same position." - page 4 in paper
        return 0;
    }

    /**
     * 
     * @param {Identifier} identifier 
     */
    addIdentifier(identifier) {
        this.identifiers.push(identifier);
    }

    toString() {
        return `{ Identifiers: [${this.identifiers.map(id => id.toString()).join(', ')}], Clock: ${this.clock} }`;
    }
}

/**
 * The logoot document is composed of "lines", where each line is a pair: <pid, atom>, where
 * pid is a position identifier and an atom can be a character or a string.
 */
class LogootDocument {
    /**
     * Constructs a new LogootDocument.
     * Initializes with a beginning and an end line that serve as boundary markers.
     */
    constructor() {
        this.lines = [
            {position: new PositionIdentifier([new Identifier(0, -1)], -Infinity), atom: null}, // LB
            {position: new PositionIdentifier([new Identifier(Infinity, -1)], Infinity), atom: null}, // LE
        ];
    }

    /**
     * Generates a new positionIdentifier between prevPos and nextPos for this siteId
     * @param {number} siteId 
     * @param {PositionIdentifier} prevPos 
     * @param {PositionIdentifier} nextPos 
     * @returns {PositionIdentifier} 
    */
    generatePosition(siteId, prevPos, nextPos) {
        let prevHead = prevPos.identifiers[0];
        let nextHead = nextPos.identifiers[0];
        console.log(`PREVHEAD: ${prevHead}`)
        console.log(`NEXTHEAD: ${nextHead}`)

        switch (prevHead.compareTo(nextHead)) {
            case -1: {
                let diff = nextHead.pos - prevHead.pos;
                if (diff > 1) {
                    return new PositionIdentifier( [ new Identifier(randomIntBetween(prevHead.pos, nextHead.pos), siteId)], 0);
                } else if (diff === 1 && siteId > prevHead.siteId) {
                    return new PositionIdentifier( [ new Identifier(prevHead.pos, siteId)], 0);
                } else {
                    let prevWithoutFirstElem = new PositionIdentifier(prevPos.identifiers.slice(1), prevPos.clock);
                    let nextWithoutFirstElem = new PositionIdentifier(nextPos.identifiers.slice(1), nextPos.clock);
                    return new PositionIdentifier([prevHead].concat(this.generatePosition(siteId, prevWithoutFirstElem, nextWithoutFirstElem).identifiers), 0);
                }
            } case 0: {
                let prevWithoutFirstElem = new PositionIdentifier(prevPos.identifiers.slice(1), prevPos.clock);
                let nextWithoutFirstElem = new PositionIdentifier(nextPos.identifiers.slice(1), nextPos.clock);
                return new PositionIdentifier([prevHead].concat(this.generatePosition(siteId, prevWithoutFirstElem, nextWithoutFirstElem).identifiers), 0);
            } case 1: {
                throw new Error('"Next" position was less than "previous" position.')
            }
        }
    }

    /**
     * Inserts an atom at a given position.
     * @param {PositionIdentifier} position - The position identifier where the atom should be inserted.
     * @param {any} atom - The content (atom) to be inserted at the position.
     */
    insert(position, atom) {
        // Find the correct position to insert the new line, ignoring the boundary markers
        let index = this.lines.findIndex(line => line.position.compareTo(position) > 0);
        this.lines.splice(index, 0, { position, atom });
    }

    /**
     * Deletes a line at a given position.
     * @param {PositionIdentifier} position - The position identifier of the line to delete.
     */
    delete(position) {
        // Find the line with the exact position and remove it
        const index = this.lines.findIndex(line => line.position.compareTo(position) === 0);
        if (index > 0 && index < this.lines.length - 1) { // Ensure not deleting boundary markers
            this.lines.splice(index, 1);
        }
    }

    /**
     * Merges another LogootDocument into this one.
     * @param {LogootDocument} other - The other LogootDocument to merge.
     */
    merge(other) {
        other.lines.forEach(line => {
            if (line.position.identifiers[0].pos !== Infinity && line.position.identifiers[0].pos !== 0) {
                const index = this.lines.findIndex(existingLine => existingLine.position.compareTo(line.position) === 0);
                if (index === -1) {
                    this.insert(line.position, line.atom);
                }

                // No action needed if the exact position already exists; assumes idempotency of operations
            }
        });
    }

    /**
     * Converts the document to a string by concatenating all atoms.
     * @returns {string} - The string representation of the document.
     */
    toString() {
        return this.lines.map(line => `position: ${line.position.toString()}, atom: ${line.atom}`).join('\n');
    }
}


/**
 * Return a random number between two others.
 *
 * @function
 * @private
 * @param {number} min The floor (random will be greater-than)
 * @param {number} max The ceiling (ranodm will be less-than)
 * @returns {number}
 */
function randomIntBetween(min, max) {
    return Math.floor(Math.random() * (max - (min + 1))) + min + 1;
  }

let iden = new Identifier(0, 1);
console.log(iden.toString());
let iden2 = new Identifier(2, 3);
console.log(iden2);
console.log(iden.compareTo(iden2))
let posIden1 = new PositionIdentifier([ iden, iden2], 1);
console.log(`posIden1 at beginning: ${posIden1}`)
posIden1.addIdentifier(new Identifier(5, 5));
console.log(`posIden1 after add: ${posIden1.toString()}`)
let posIden2 = new PositionIdentifier([
    new Identifier(0, 1),
    new Identifier(2, 3),
    new Identifier(4, 5)
], 2);
console.log(`posIden2: ${posIden2.toString()}`);
console.log(posIden1.compareTo(posIden2));

let logoot = new LogootDocument();
console.log(`\nlogoot at beginning:\n${logoot.toString()}\n`);
logoot.insert(posIden1, "aa");
console.log(`\nlogoot:\n${logoot.toString()}\n`);
logoot.insert(posIden2, "bb")
console.log(`\nlogoot:\n${logoot.toString()}\n`);
let newPos = logoot.generatePosition(10, posIden2, posIden1);
console.log(`newPos: ${newPos.toString()}`);
logoot.insert(newPos, "ccc");
console.log(`\nlogoot:\n${logoot.toString()}\n`);