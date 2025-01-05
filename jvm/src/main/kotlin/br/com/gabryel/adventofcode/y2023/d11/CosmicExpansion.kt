package br.com.gabryel.adventofcode.y2023.d11

import br.com.gabryel.adventofcode.util.CharArray2D
import br.com.gabryel.adventofcode.y2023.size

fun calculateCosmicExpansionMinimumDistanceOfPairs(lines: List<String>, scale: Long) =
    lines.map { it.toCharArray() }.toTypedArray().calculateMinimumDistanceOfPairs(scale)

private fun CharArray2D.calculateMinimumDistanceOfPairs(scale: Long): Long {
    val galaxies = flatMapIndexed { indexX, row ->
        row.mapIndexed { indexY, space -> if (space == '#') indexY else null }
            .filterNotNull()
            .map { indexX to it }
    }

    val filledRows = galaxies.map { it.first }.toSet()
    val filledCols = galaxies.map { it.second }.toSet()

    return galaxies.indices.sumOf { aIndex ->
        val (aRow, aCol) = galaxies[aIndex]
        (aIndex + 1..galaxies.lastIndex).sumOf { bIndex ->
            val (bRow, bCol) = galaxies[bIndex]

            calculateDistance(aCol, bCol, filledCols, scale) +
                    calculateDistance(aRow, bRow, filledRows, scale)
        }
    }
}

private fun calculateDistance(galaxyA: Int, galaxyB: Int, filled: Set<Int>, scale: Long): Long {
    val sorted = listOf(galaxyA, galaxyB).sorted()
    val (first, second) = sorted

    val galaxyRange = first until second
    val hasGalaxy = galaxyRange.count { it in filled }
    val hasNoGalaxy = galaxyRange.size() - hasGalaxy

    return hasGalaxy + (hasNoGalaxy * scale)
}
