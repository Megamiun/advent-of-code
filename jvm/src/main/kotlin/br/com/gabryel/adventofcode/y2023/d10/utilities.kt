package br.com.gabryel.adventofcode.y2023.d10

fun CoordMap.findOnPosition(origin: Coordinate) =
    getOrNull(origin.first)
        ?.getOrNull(origin.second)
        ?: '~'
