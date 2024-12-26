package br.com.gabryel.adventofcode.y2023.d21.area

import br.com.gabryel.adventofcode.util.*
import kotlin.math.pow

typealias StepState = Pair<Long, Coordinate>
typealias AreaState = Pair<Long, Area>

interface Area {
    class Context(val map: CharArray2D, private val known: MutableMap<Pair<Int, List<StepState>>, Area> = mutableMapOf()) {
        val levelFactor = 3

        val halfLevelFactor = levelFactor / 2

        private val levelFactorFloat = levelFactor.toFloat()

        private val dimension = map.size

        val dimensions = map[0].size to map.size

        fun getLevelMultiplier(level: Int) = (levelFactorFloat.pow(level - 2)).toLong() * dimension

        fun get(area: Area, direction: Direction, calculate: () -> Area) =
            known.getOrPut(area.level to area.getSignals(direction)) { calculate() }

        fun expandToAtLeast(width: Int): Context {
            val multiplier = (width / dimension) + 1
            val newDimension = dimension * multiplier

            if (multiplier == 1) return this

            val newMap = Array(newDimension) { y ->
                CharArray(newDimension) { x -> map[(x to y) bindTo dimensions]!! }
            }

            return Context(newMap)
        }
    }

    val context: Context

    val stepsToEnd: Long

    val firstSignal: Long

    val level: Int

    fun getSignals(direction: Direction): List<StepState>

    fun getTimeToSignal(direction: Direction): Long

    fun countPossibleAtStep(steps: Long): Long

    fun expand(direction: Direction): Area

    fun grow() = MultiField.growFrom(mapOf((0 to 0) to (0L to this)))
}
