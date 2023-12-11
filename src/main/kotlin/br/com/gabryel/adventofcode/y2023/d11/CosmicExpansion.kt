package br.com.gabryel.adventofcode.y2023.d11

import br.com.gabryel.adventofcode.y2023.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val map = readLines(11, file).map { it.toCharArray() }.toTypedArray()

        listOf<Long>(2, 10, 100, 1000000).forEach {
            val padded = it.toString().padEnd(10)
            println("[Sum Of Minimum Distances][$padded][$file] ${map.calculateMinimumDistanceOfPairs(it)}")
        }
    }
}

private fun Array<CharArray>.calculateMinimumDistanceOfPairs(scale: Long = 2): Long {
    val galaxies = mapIndexed { indexX, row ->
        row
            .mapIndexed { indexY, space -> if (space == '#') indexY else null }
            .filterNotNull()
            .map { indexX to it }
    }.flatten()

    val filledRows = galaxies.map { it.first }.toSet()
    val filledCols = galaxies.map { it.second }.toSet()

    val result = galaxies.indices.sumOf { aIndex ->
        val (aX, aY) = galaxies[aIndex]
        (aIndex + 1..galaxies.lastIndex).sumOf { bIndex ->
            val (bX, bY) = galaxies[bIndex]

            calculateDistance(aY, bY, filledCols, scale) +
                    calculateDistance(aX, bX, filledRows, scale)
        }
    }
    return result
}

private fun calculateDistance(left: Int, right: Int, filled: Set<Int>, scale: Long): Long {
    val sorted = listOf(left, right).sorted()
    val (first, second) = sorted

    return (second - first) + ((first..second).count { it !in filled } * (scale - 1))
}
