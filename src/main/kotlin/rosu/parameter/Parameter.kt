package rosu.parameter

import java.nio.ByteBuffer

interface Parameter {
    fun size(): Int
    fun toBytes(): ByteArray
    fun buffer(action:ByteBuffer.() -> Unit): ByteArray {
        val buffer = ByteBuffer.allocate(size())
        buffer.action()
        return buffer.array()
    }
}