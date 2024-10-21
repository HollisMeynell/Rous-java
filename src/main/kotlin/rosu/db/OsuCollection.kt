package rosu.db

import rosu.OsuDB
import rosu.db.OsuCollection.CollectionItem
import java.lang.ref.Cleaner

@Suppress("unused")
class OsuCollection internal constructor() : AutoCloseable, Iterable<CollectionItem>, MutableIterator<CollectionItem> {
    var version: Int = 0
        internal set
    private var items = mutableListOf<CollectionItem>()
    private var ptr = 0L
        get() {
            if (field == 0L) throw IllegalAccessException("Collection is released or not ready")
            return field
        }
    private var cleaner = CLEANER.register(this) {
        OsuDB.releaseCollectionList(ptr)
    }

    internal fun setCollections(data: String) {
        data.split("|").forEachIndexed { index, it ->
            val i = it.indexOf("#")
            if (i < 0) return@forEachIndexed
            val collectionName = it.substring(0, i)
            val itemsStr = it.substring(i + 1, it.length - 1)
            if (itemsStr.isEmpty()) return@forEachIndexed

            val item = CollectionItem(
                collectionName, ArrayList(itemsStr.split(",")), index, this
            )
            items.add(item)
        }
    }

    fun addCollection(name: String, hashed: Iterable<String>): CollectionItem {
        val item = CollectionItem(
            name, hashed.toCollection(ArrayList()), items.size, this
        )
        OsuDB.addCollection(ptr, name, hashed)
        items.add(item)
        return item
    }

    fun addCollection(name: String): CollectionItem {
        val item = CollectionItem(
            name, ArrayList(), items.size, this
        )
        OsuDB.addCollection(ptr, name)
        items.add(item)
        return item
    }

    internal fun removeCollection(index: Int) {
        if (index !in items.indices) return
        OsuDB.removeCollection(ptr, index)
        items.removeAt(index)
    }

    internal fun clearCollections(index: Int) {
        OsuDB.clearCollection(ptr, index)
    }

    internal fun addAllHash(index: Int, hash: Iterable<String>) {
        OsuDB.addAllCollectionHash(ptr, index, hash)
    }

    internal fun setCollectionName(index: Int, name: String) {
        OsuDB.setCollectionName(ptr, index, name)
    }

    internal fun appendHash(index: Int, hash: String) {
        OsuDB.appendCollectionHash(ptr, index, hash)
    }

    internal fun insertHash(index: Int, hashIndex: Int, hash: String) {
        OsuDB.insertCollectionHash(ptr, index, hashIndex, hash)
    }

    internal fun setHash(index: Int, hashIndex: Int, hash: String) {
        OsuDB.setCollectionHash(ptr, index, hashIndex, hash)
    }

    internal fun removeHash(index: Int, hashIndex: Int) {
        OsuDB.removeCollectionHash(ptr, index, hashIndex)
    }

    fun toBytes(): ByteArray {
        return OsuDB.toBytes(ptr)
    }

    class CollectionItem(
        name: String,
        private val hashes: ArrayList<String>,
        private val index: Int,
        private val collection: OsuCollection,
    ) : Iterable<String>, MutableIterator<String> {
        private var name = name
            set(value) {
                field = value
                collection.setCollectionName(index, name)
            }

        fun getHashes(): Array<String> {
            return hashes.toTypedArray()
        }

        fun clearHashes() {
            hashes.clear()
            collection.clearCollections(index)
        }

        fun addAllHashes(hashes: Iterable<String>) {
            this.hashes.addAll(hashes)
            collection.addAllHash(index, hashes)
        }

        fun setHash(index: Int, hash: String) {
            if (index in hashes.indices) {
                hashes[index] = hash
                collection.setHash(this.index, index, hash)
            }
        }

        fun appendHash(hash: String) {
            hashes.add(hash)
            collection.appendHash(index, hash)
        }

        fun insertHash(index: Int, hash: String) {
            hashes.add(index, hash)
            collection.insertHash(this.index, index, hash)
        }

        fun removeHash(hash: String) {
            hashes.indexOfFirst { it == hash }.let {
                if (it < 0) return
                hashes.removeAt(it)
                collection.removeHash(index, it)
            }

        }

        fun removeHash(index: Int) {
            if (index in hashes.indices) {
                hashes.removeAt(index)
                collection.removeHash(this.index, index)
            }
        }

        /**
         * Returns an iterator over the elements of this object.
         */
        override fun iterator(): Iterator<String> = hashes.iterator()

        private val iterator by lazy { hashes.iterator() }

        /**
         * Returns `true` if the iteration has more elements.
         * (In other words, returns `true` if [.next] would
         * return an element rather than throwing an exception.)
         *
         * @return `true` if the iteration has more elements
         */
        override fun hasNext() = iterator.hasNext()

        /**
         * Returns the next element in the iteration.
         *
         * @return the next element in the iteration
         * @throws NoSuchElementException if the iteration has no more elements
         */
        override fun next() = iterator.next()

        override fun remove() = iterator.remove()

    }

    override fun close() {
        try {
            ptr
        } catch (e: IllegalAccessException) {
            return
        }
        cleaner.clean()
        ptr = 0
    }

    override fun iterator(): Iterator<CollectionItem> {
        return items.iterator()
    }

    private val iterator by lazy { items.iterator() }

    override fun hasNext() = this.iterator.hasNext()

    override fun next() = iterator.next()

    override fun remove() = iterator.remove()

    private companion object {
        private val CLEANER = Cleaner.create()
    }
}