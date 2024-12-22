package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.getLinesFromSystemIn

fun main() {
    val summary = getSummaryOfDirs()
    println("Sum of Dirs With Size <= 100000: ${summary.filterValues { it <= 100000 }.values.sum()}")
    val spaceNeededToFree = 30000000 - (70000000 - (summary["/"] ?: 0))
    println("Min Size of Dir Which Leaves 30000000 Free: ${summary.values.sorted().first { it > spaceNeededToFree }}")
}

private fun getSummaryOfDirs(): Map<String, Long> {
    val linesIterator = getLinesFromSystemIn().toList().listIterator()
    val acc = Accumulator(currDir = "/").parse(linesIterator)

    acc.printPath()
    println()

    val summary = mutableMapOf<String, Long>()
    acc.mountDirSizes("/", summary)

    return summary
}

private fun Accumulator.mountDirSizes(path: String, summary: MutableMap<String, Long>): Long {
    val fileSize = fileSizes[path]
    if (fileSize != null) return fileSize

    val filesTotal = directoryChildren[path].orEmpty().sumOf { newPath -> mountDirSizes(newPath, summary) }
    summary[path] = filesTotal
    return filesTotal
}

tailrec fun Accumulator.parse(iterator: ListIterator<String>): Accumulator {
    if (!iterator.hasNext()) return this

    val current = iterator.next()
    return executeCommand(current.removePrefix("$ "), iterator)
        .parse(iterator)
}

private fun Accumulator.executeCommand(current: String, iterator: ListIterator<String>): Accumulator {
    if (current.startsWith("cd"))
        return cd(current.removePrefix("cd "))

    return generateSequence {
        iterator.nextIfMatches { !it.startsWith("$") }
    }.fold(this) { acc, line -> acc.with(line) }
}

private fun <T> ListIterator<T>.nextIfMatches(predicate: (T) -> Boolean) =
    if (hasNext()) {
        val next = next()
        if (!predicate(next)) {
            previous()
            null
        } else next
    } else null

data class Accumulator(
    val directoryChildren: Map<String, List<String>> = mapOf(),
    val fileSizes: Map<String, Long> = mapOf(),
    val currDir: String
) {
    fun cd(command: String) = when (command) {
        "/" -> copy(currDir = "/")
        ".." -> copy(currDir = currDir.replace("(.*/).+?/".toRegex(), "$1"))
        else -> copy(currDir = "$currDir$command/")
    }

    fun with(line: String): Accumulator {
        val (first, second) = line.split(" ")

        val currentSubs = directoryChildren[currDir].orEmpty()
        val newPath = "$currDir$second"

        return when (first) {
            "dir" -> copy(directoryChildren = directoryChildren + (currDir to (currentSubs + "$newPath/")))
            else -> copy(
                directoryChildren = directoryChildren + (currDir to (currentSubs + newPath)),
                fileSizes = fileSizes + (newPath to first.toLong())
            )
        }
    }

    fun printPath(path: String = "/", level: Int = 0) {
        repeat(level) { print("  ") }
        println("- ${path.replace(".*/(.+)/".toRegex(), "$1")} (dir)")

        directoryChildren[path].orEmpty().forEach { newPath ->
            val file = fileSizes[newPath]
                ?: return@forEach printPath(newPath, level + 1)

            repeat(level + 1) { print("  ") }
            println("- ${newPath.replace(".*/(.+)".toRegex(), "$1")} (file, size=$file)")
        }
    }
}
