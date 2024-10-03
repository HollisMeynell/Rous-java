package rosu

import rosu.db.OsuCollection
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardCopyOption
import kotlin.io.path.Path
import kotlin.io.path.absolutePathString
import kotlin.io.path.createDirectories

internal class Native private constructor() {
    companion object {
        @JvmStatic
        val instance by lazy {
            val native = Native()
            native.loadLib
            native
        }
    }

    val loadLib by lazy {
        System.getenv("ROSU_LIB_PATH")?.let {
            if (Files.isRegularFile(Path(it))) {
                System.load(it)
                return@lazy
            }
        }
        val os: String = System.getProperty("os.name")
        val name = when {
            os.contains("windows", ignoreCase = true) -> "rosu_pp_java.dll"
            os.contains("mac", ignoreCase = true) -> "librosu_pp_java.dylib"
            os.contains("linux", ignoreCase = true) -> "librosu_pp_java.so"
            else -> throw Error("Unsupported OS")
        }
        val lib = Native::class.java.getResourceAsStream("/lib/${name}")
        lib?.use {
            val tmpDirPath = Path.of(
                System.getenv("ROSU_LIB_PATH") ?: (System.getProperty("java.io.tmpdir") + "/jlib")
            )
            tmpDirPath.createDirectories()
            val f = tmpDirPath.resolve(name)
            Files.copy(it, f, StandardCopyOption.REPLACE_EXISTING)
            f.toFile().deleteOnExit()
            System.load(f.absolutePathString())
        }
    }

    @JvmName("calculate")
    external fun calculate(localMap: ByteArray, source: ByteArray): ByteArray

    @JvmName("getCalculateIterator")
    external fun getCalculateIterator(localMap: ByteArray, mapAttr: ByteArray): ByteArray

    @JvmName("calculateIterator")
    external fun calculateIterator(ptr: Long, score: ByteArray): ByteArray

    @JvmName("releaseCalculate")
    external fun releaseCalculate(ptr: Long): ByteArray

    /**********************************************************************************************/
    @JvmName("createCollection")
    external fun createCollection(collection: OsuCollection): ByteArray

    @JvmName("readCollection")
    external fun readCollection(data: ByteArray, collection: OsuCollection): ByteArray

    @JvmName("writeCollection")
    external fun writeCollection(ptr: Long): ByteArray

    // [(!0 error, 0 ok)-data]
    @JvmName("releaseCollection")
    external fun releaseCollection(ptr: Long): ByteArray

    // hashes: "hash1,hash2,hash3"
    @JvmName("addCollection")
    external fun addCollection(ptr: Long, name: String, hashes: String): ByteArray

    @JvmName("removeCollection")
    external fun removeCollection(ptr: Long, index: Int): ByteArray

    @JvmName("clearCollection")
    external fun clearCollection(ptr: Long, index: Int): ByteArray

    @JvmName("addAllCollectionHash")
    external fun addAllCollectionHash(ptr: Long, index: Int, hashes: String): ByteArray

    @JvmName("setCollectionName")
    external fun setCollectionName(ptr: Long, index: Int, name: String): ByteArray

    @JvmName("appendCollectionHash")
    external fun appendCollectionHash(ptr: Long, index: Int, hash: String): ByteArray

    @JvmName("insertCollectionHash")
    external fun insertCollectionHash(ptr: Long, index: Int, hashIndex: Int, hash: String): ByteArray

    @JvmName("setCollectionHash")
    external fun setCollectionHash(ptr: Long, index: Int, hashIndex: Int, hash: String): ByteArray

    @JvmName("removeCollectionHash")
    external fun removeCollectionHash(ptr: Long, index: Int, hashIndex: Int): ByteArray

}