package br.com.gabryel.adventofcode.y2022.d07

import br.com.gabryel.adventofcode.util.PeekableIterator
import br.com.gabryel.adventofcode.util.peekingIterator

private val REMOVE_LAST = "(.*/).+?/".toRegex()

fun getSummaryOfDirsLessThan(lines: List<String>, max: Int) =
    lines.getSummaryOfDirs().filterValues { it <= max }.values.sum()

fun getSummaryOfDirsBetween(lines: List<String>, min: Int, max: Int): Int {
    val summary = lines.getSummaryOfDirs()
    val spaceNeededToFree = min - (max - (summary["/"] ?: 0))

    return summary.values.sorted().first { it > spaceNeededToFree }
}

private fun List<String>.getSummaryOfDirs(): Map<String, Int> {
    val acc = FileSystem(currDir = "/").parse(peekingIterator())
    acc.mountDirSizes("/")

    return acc.cache
}

private fun FileSystem.mountDirSizes(path: String): Int {
    val fileSize = fileSizes[path]
    if (fileSize != null) return fileSize

    val filesTotal = dirMap[path].orEmpty().sumOf(::mountDirSizes)
    cache[path] = filesTotal
    return filesTotal
}

tailrec fun FileSystem.parse(iterator: PeekableIterator<String>): FileSystem {
    if (!iterator.hasNext()) return this

    val current = iterator.next()
    return executeCommand(current.removePrefix("$ "), iterator)
        .parse(iterator)
}

private fun FileSystem.executeCommand(current: String, iterator: PeekableIterator<String>): FileSystem {
    if (current.startsWith("cd"))
        return cd(current.removePrefix("cd "))

    return generateSequence {
        iterator.nextIfMatches { !it.startsWith("$") }
    }.fold(this) { acc, line -> acc.with(line) }
}

data class FileSystem(
    val dirMap: Map<String, List<String>> = mapOf(),
    val fileSizes: Map<String, Int> = mapOf(),
    val currDir: String
) {
    val cache: MutableMap<String, Int> = mutableMapOf()

    fun cd(to: String) = when (to) {
        "/" -> copy(currDir = "/")
        ".." -> copy(currDir = currDir.replace(REMOVE_LAST, "$1"))
        else -> copy(currDir = "$currDir$to/")
    }

    fun with(content: String): FileSystem {
        val (first, second) = content.split(" ")

        val currentSubs = dirMap[currDir].orEmpty()
        val newPath = "$currDir$second"

        return when (first) {
            "dir" -> copy(dirMap = dirMap + (currDir to (currentSubs + "$newPath/")))
            else -> copy(
                dirMap = dirMap + (currDir to (currentSubs + newPath)),
                fileSizes = fileSizes + (newPath to first.toInt())
            )
        }
    }
}
