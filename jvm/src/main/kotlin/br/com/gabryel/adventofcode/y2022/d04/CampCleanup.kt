package br.com.gabryel.adventofcode.y2022.d04

import br.com.gabryel.adventofcode.util.intersects
import br.com.gabryel.adventofcode.util.isInside

fun getFullyContainedRanges(lines: List<String>) = lines.parse()
    .count { (first, second) -> first isInside second || second isInside first }

fun getOverlappingRanges(lines: List<String>) = lines.parse()
    .count { (first, second) -> first intersects second }

private fun List<String>.parse() = map { lines ->
    val (first, second) = lines.split(",").map { pair ->
        val (start, ending) = pair.split("-").map { it.toInt() }
        start..ending
    }

    first to second
}
