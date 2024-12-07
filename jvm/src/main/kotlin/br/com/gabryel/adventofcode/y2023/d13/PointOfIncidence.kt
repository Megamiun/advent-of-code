package br.com.gabryel.adventofcode.y2023.d13

import br.com.gabryel.adventofcode.readGroupsOfLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val terrains = readGroupsOfLines(2023, 13, file)

        listOf(0, 1).forEach {
            val padded = it.toString().padEnd(3)
            println("[Sum Of Possibilities][$padded][$file] ${terrains.sumOfIncidencePoints(it)}")
        }
    }
}

private fun List<List<String>>.sumOfIncidencePoints(smudges: Int) = sumOf { terrain ->
    val horizontalCut = findReflectionLine(terrain.first().lastIndex, smudges, terrain::extractColumn) ?: 0
    val verticalCut = findReflectionLine(terrain.lastIndex, smudges, terrain::get) ?: 0

    verticalCut * 100 + horizontalCut
}

private fun List<String>.extractColumn(curr: Int) = joinToString("") { row -> row[curr].toString() }

private fun findReflectionLine(lastIndex: Int, smudges: Int, extractVectorAt: (Int) -> String) =
    (0 until lastIndex).firstOrNull { halfPoint ->
        val mirroredHalf = (0 .. halfPoint).map(extractVectorAt).reversed()
        val preservedHalf = (halfPoint + 1 .. lastIndex).map(extractVectorAt)

        val errorsFound = mirroredHalf.zip(preservedHalf)
            .sumOf { (mirrored, preserved) ->
                mirrored.zip(preserved)
                    .count { (mirroredChar, preservedChar) -> mirroredChar != preservedChar }
            }

        errorsFound == smudges
    }?.let { it + 1 }
