package rosu

import rosu.db.OsuCollection


object OsuDB {
    private val native = Native.instance

    @JvmStatic
    @Suppress("unused")
    fun createCollection(): OsuCollection {
        val collection = OsuCollection()
        native.createCollection(collection).handle()
        return collection
    }

    @JvmStatic
    @Suppress("unused")
    fun readCollection(data: ByteArray): OsuCollection {
        val collection = OsuCollection()
        native.readCollection(data, collection).handle()
        return collection
    }

    internal fun releaseCollectionList(ptr: Long) {
        native.releaseCollection(ptr).handle()
    }

    internal fun toBytes(ptr: Long): ByteArray {
        val bytes = native.writeCollection(ptr)
        return JniProcessor.readJniBytes(bytes)
    }

    internal fun addCollection(ptr: Long, name: String, hashes: Iterable<String>?) {
        val hashesStr = hashes?.joinToString(",") ?: ""
        native.addCollection(ptr, name, hashesStr).handle()
    }

    internal fun addCollection(ptr: Long, name: String) {
        addCollection(ptr, name, null)
    }

    internal fun removeCollection(ptr: Long, index: Int) {
        native.removeCollection(ptr, index).handle()
    }

    internal fun clearCollection(ptr: Long, index: Int) {
        native.clearCollection(ptr, index).handle()
    }

    internal fun addAllCollectionHash(ptr: Long, index: Int, hashes: Iterable<String>) {
        val hashesStr = hashes.joinToString(",")
        if (hashesStr.isEmpty()) return
        native.addAllCollectionHash(ptr, index, hashesStr).handle()
    }

    internal fun setCollectionName(ptr: Long, index: Int, name: String) {
        native.setCollectionName(ptr, index, name).handle()
    }

    internal fun appendCollectionHash(ptr: Long, index: Int, hash: String) {
        native.appendCollectionHash(ptr, index, hash).handle()
    }

    internal fun insertCollectionHash(ptr: Long, index: Int, hashIndex: Int, hash: String) {
        native.insertCollectionHash(ptr, index, hashIndex, hash).handle()
    }

    internal fun setCollectionHash(ptr: Long, index: Int, hashIndex: Int, name: String) {
        native.setCollectionHash(ptr, index, hashIndex, name).handle()
    }

    internal fun removeCollectionHash(ptr: Long, index: Int, hashIndex: Int) {
        native.removeCollectionHash(ptr, index, hashIndex).handle()
    }


    private inline fun ByteArray.handle() {
        if (this.isNotEmpty()) throw Exception(String(this))
    }
}