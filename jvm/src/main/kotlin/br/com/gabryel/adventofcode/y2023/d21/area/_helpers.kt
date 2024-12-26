package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.getManhattanDistance

fun List<StepState>.filterRedundant() = filter { (lDistance, lCoord) ->
    none { (rDistance, rCoord) ->
        lCoord != rCoord && lCoord.getManhattanDistance(rCoord) == (lDistance - rDistance).toInt()
    }
}
