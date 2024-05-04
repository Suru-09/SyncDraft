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
     * @param {number} pos
     * @param {number} siteId
     */
    constructor(pos, siteId) {
        this.pos = pos;
        this.siteId = siteId;
        
    }

    toString() {
        return `<${this.pos},${this.siteId}>`;
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