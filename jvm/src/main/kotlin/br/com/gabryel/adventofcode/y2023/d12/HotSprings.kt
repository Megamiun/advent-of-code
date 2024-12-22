package br.com.gabryel.adventofcode.y2023.d12

import br.com.gabryel.adventofcode.util.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(2023, 12, file)

        listOf(1, 5).forEach {
            val padded = it.toString().padEnd(3)
            println("[Sum Of Possibilities][$padded][$file] ${lines.findNumberOfPossibilities(it)}")
        }
    }
}

private fun List<String>.findNumberOfPossibilities(foldingLevel: Int) = sumOf { line ->
    val (originalReport, originalDamageMap) = line.split(" ")

    PossibilityCounter(originalReport, originalDamageMap, foldingLevel).count()
}

private class PossibilityCounter(originalReport: String, originalDamageMap: String, foldingLevel: Int) {

    private val cache: MutableMap<Triple<Int, Int, Boolean>, Long> = mutableMapOf()

    private val report = originalReport.unfold(foldingLevel, "?")

    private val damageMap = originalDamageMap.unfold(foldingLevel, ",")
        .split(",").map { it.toInt() }

    fun count(stringIndex: Int = 0, damagedIndex: Int = 0, skipNext: Boolean = false): Long {
        val remainingString = report.substring(stringIndex)
        if (damagedIndex > damageMap.lastIndex) {
            return if ("#" in remainingString) 0 else 1
        }

        if (stringIndex > report.lastIndex) return 0

        val possiblyDamagedSpacesAhead = remainingString.takeWhile { it in "#?" }.length
        val currentDamagedSize = damageMap[damagedIndex]

        val onSkip =
            if (report[stringIndex] != '#') countPossibilitiesCached(stringIndex + 1, damagedIndex)
            else 0

        if (skipNext) return onSkip
        if (possiblyDamagedSpacesAhead < currentDamagedSize) return onSkip

        val onDamage = countPossibilitiesCached(stringIndex + currentDamagedSize, damagedIndex + 1, true)
        return onSkip + onDamage
    }

    private fun countPossibilitiesCached(stringIndex: Int, damagedIndex: Int, skipNext: Boolean = false): Long {
        val cacheKey = Triple(stringIndex, damagedIndex, skipNext)
        val cached = cache[cacheKey]

        if (cached != null) return cached

        return count(stringIndex, damagedIndex, skipNext).apply {
            cache[cacheKey] = this
        }
    }

    private fun String.unfold(foldingLevel: Int, separator: String) =
        (0 until foldingLevel).joinToString(separator) { this }
}