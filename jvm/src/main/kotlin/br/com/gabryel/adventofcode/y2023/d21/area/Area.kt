package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.*
import kotlin.math.pow

typealias StepState = Pair<Int, Coordinate>
typealias AreaState = Pair<Int, Area>

interface Area {
    class Context(val map: CharArray2D, private val known: MutableMap<Pair<Int, List<StepState>>, Area> = mutableMapOf()) {
        val levelFactor = 4

        private val levelFactorFloat = levelFactor.toFloat()

        private val dimension = map.size

        val dimensions = map[0].size to map.size

        val multiDimensions = levelFactor to levelFactor

        fun getLevelMultiplier(level: Int) = (levelFactorFloat.pow(level - 2)).toInt() * dimension

        fun get(area: Area, direction: Direction, calculate: () -> Area) =
            known.getOrPut(area.level to area.getSignals(direction)) { calculate() }
    }

    val context: Context

    val stepsToEnd: Int

    val firstSignal: Int

    val level: Int

    fun getSignals(direction: Direction): List<StepState>

    fun getTimeToSignal(direction: Direction): Int

    fun countPossibleAtStep(steps: Int): Long

    fun expand(direction: Direction): Area

    fun grow() = MultiField.growFrom(mapOf((context.multiDimensions / 2) to (0 to this)))
}
