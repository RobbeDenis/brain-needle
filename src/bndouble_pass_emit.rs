
// bytecode interpreter ???
// example
/* Instructions can have extra value attached and is displayed like -> Size      Value   */
pub enum Instruction {
    /* Base instrucitons - expected to be supported by eevery emitter */
    BAdd,           /*IValue    Amount  */
    BSub,           /*IValue    Amount  */
    BRight,         /*IValue    Amount  */
    BLeft,          /*IValue    Amount  */
    BLoop,          /*IValue    Index   */
    BEndLoop,       /*IValue    Index   */
    BIn,
    BOut,

    /* Direct output */
    ByteOut,        /*u8        Byte    */
    ByteOutN,       /*u8        Byte    */

    /* Metadata - is used in combination with other instructions */
    N,              /*IValue    Amount - always comes after the instruciton that needs it */
}

pub trait BNDoublePassEmitter<BNAnalysisPass, BNEmitPass> {
    fn analyze();
}

pub trait BNAnalysisPass {

}

pub trait BNEmitPass {
    
}