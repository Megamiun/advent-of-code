package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.CharMap
import br.com.gabryel.adventofcode.util.Coordinate
import br.com.gabryel.adventofcode.util.Direction
import kotlin.math.pow

typealias StepState = Pair<Long, Coordinate>
typealias AreaState = Pair<Long, Area>

interface Area {
    class Context(val map: CharMap, private val known: MutableMap<Pair<Int, List<StepState>>, Area>) {
        val levelFactor = 7

        val halfLevelFactor = levelFactor / 2

        private val levelFactorFloat = levelFactor.toFloat()

        private val dimension = map.size

        val dimensions = map[0].size to map.size

        fun getLevelMultiplier(level: Int) = (levelFactorFloat.pow(level - 2)).toLong() * dimension

        fun get(area: Area, direction: Direction, calculate: () -> Area) =
            known.getOrPut(area.level to area.getSignals(direction)) { calculate() }
    }

    val context: Context

    val stepsToEnd: Long

    val firstOut: Long

    val level: Int

    fun getSignals(direction: Direction): List<StepState>

    fun getTimeToSignal(direction: Direction): Long

    fun countPossibleAtStep(steps: Long): Long

    fun expand(direction: Direction): Area

    fun grow() = MultiField.growFrom(mapOf((0 to 0) to (0L to this)))

    fun getPossiblePerParity(even: Boolean): Long
}
