package rosu.parameter

interface Parameter {
    fun size(): Int
    fun toBytes(): ByteArray
}