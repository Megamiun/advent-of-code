package br.com.gabryel.adventofcode.y2023

object ResourceReader

fun readGroupsOfLines(day: Int, file: String): List<List<String>> {
    val iterator = readLines(day, file, keepBlanks = true).listIterator()

    return generateSequence { iterator.takeUntilNextBlankLine(false) }
        .takeWhile { it.isNotEmpty() }
        .toList()
}

fun ListIterator<String>.takeUntilNextBlankLine(hasHeader: Boolean = true): List<String> {
    val result = generateSequence { nextIfMatches(String::isNotBlank) }.toList()
    if (hasNext()) next() // Just to move the cursor

    return if (hasHeader) result.drop(1)
    else result
}

fun readLines(day: Int, file: String, keepBlanks: Boolean = false) =
    readFile(day, file)
        .lines()
        .filter { keepBlanks || it.isNotBlank() }

fun readFile(day: Int, file: String) =
    ResourceReader.javaClass.getResourceAsStream("d${day.toString().padStart(2, '0')}/$file")
        ?.bufferedReader()?.readText()
        ?: throw IllegalStateException("""¨d$day/$file¨ not found""")

private fun <T> ListIterator<T>.nextIfMatches(matches: T.() -> Boolean) =
    if (hasNext()) {
        val next = next()
        if (!next.matches()) {
            previous()
            null
        } else next
    } else null
