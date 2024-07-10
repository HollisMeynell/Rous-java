package rosu

import rosu.result.JniProcessor

@Suppress("unused")
object OsuDB {
    private val native = Native.instance

    @JvmStatic
    fun createCollectionList(name: String, mapsMd5: Iterable<String>): ByteArray {
        val ptr = JniProcessor.readPointer(native.createCollection(name))
        mapsMd5.forEach {
            native.setCollectionMap(ptr, it)
        }
        return JniProcessor.readBytes(native.newCollectionList(ptr))
    }

    @JvmStatic
    fun appendCollectionList(data: ByteArray, name: String, mapsMd5: Iterable<String>): ByteArray {
        val ptr = JniProcessor.readPointer(native.createCollection(name))
        mapsMd5.forEach {
            native.setCollectionMap(ptr, it)
        }
        return JniProcessor.readBytes(native.appendCollectionList(data, ptr))
    }
}