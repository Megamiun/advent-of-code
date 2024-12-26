package br.com.gabryel.adventofcode.y2023.d10

import br.com.gabryel.adventofcode.util.CharArray2D
import br.com.gabryel.adventofcode.util.Coordinate

fun CharArray2D.findOnPosition(origin: Coordinate) =
    getOrNull(origin.first)
        ?.getOrNull(origin.second)
        ?: '~'
