package br.com.gabryel.adventofcode.y2023.d10

import br.com.gabryel.adventofcode.util.CharMap
import br.com.gabryel.adventofcode.util.Coordinate

fun CharMap.findOnPosition(origin: Coordinate) =
    getOrNull(origin.first)
        ?.getOrNull(origin.second)
        ?: '~'
