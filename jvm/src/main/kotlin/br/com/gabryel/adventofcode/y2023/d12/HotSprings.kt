package br.com.gabryel.adventofcode.y2023.d12

fun findHotSpringsPossibilities(lines: List<String>, foldingLevel: Int) = lines.sumOf { line ->
    val (originalReport, originalDamageMap) = line.split(" ")

    PossibilityCounter(originalReport, originalDamageMap, foldingLevel).count()
}

private class PossibilityCounter(originalReport: String, originalDamageMap: String, foldingLevel: Int) {

    private val cache: MutableMap<Triple<Int, Int, Boolean>, Long> = mutableMapOf()

    private val report = originalReport.unfold(foldingLevel, "?")

    private val damageMap = originalDamageMap
        .split(",").map { it.toInt() }
        .unfold(foldingLevel)

    fun count(stringIndex: Int = 0, damagedIndex: Int = 0, skipNext: Boolean = false): Long =
        cache.getOrPut(Triple(stringIndex, damagedIndex, skipNext)) {
            val remainingString = report.substring(stringIndex)
            if (damagedIndex > damageMap.lastIndex) {
                return@getOrPut if ("#" in remainingString) 0 else 1
            }

            if (stringIndex > report.lastIndex) return 0

            val possiblyDamagedSpacesAhead = remainingString.takeWhile { it in "#?" }.length
            val currentDamagedSize = damageMap[damagedIndex]

            val onSkip =
                if (report[stringIndex] != '#') count(stringIndex + 1, damagedIndex)
                else 0

            if (skipNext) return@getOrPut onSkip
            if (possiblyDamagedSpacesAhead < currentDamagedSize) return@getOrPut onSkip

            val onDamage = count(stringIndex + currentDamagedSize, damagedIndex + 1, true)

            onSkip + onDamage
        }

    private fun List<Int>.unfold(foldingLevel: Int) =
        (0 until foldingLevel).flatMap { this }

    private fun String.unfold(foldingLevel: Int, separator: String) =
        (0 until foldingLevel).joinToString(separator) { this }
}