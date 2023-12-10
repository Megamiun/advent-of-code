package br.com.gabryel.adventofcode.y2023.d08

data class NodeStep(val node: String, val step: Long)
data class NodePosition(val node: String, val step: Int)

class MultiStepMap(
    private val directions: String,
    private val mappings: Map<String, Pair<String, String>>
) {

    private val toZCache = HashMap<String, Array<NodePosition?>>()

    fun findStepsToArrive(): Long {
        val startingPoints = mappings.entries
            .filter { it.key.endsWith('A') }
            .map { NodeStep(it.key, 0L) }

        return generateSequence(startingPoints) { previousPoints ->
            val currentMax = previousPoints.maxOf { it.step }

            previousPoints.map { data ->
                val (node, step) = data
                if (step == currentMax && node.endsWith('Z')) return@map data

                generateSequence(data, ::moveUntilNextZ)
                    .drop(1)
                    .first { (_, steps) -> steps >= currentMax }
            }
        }.extractFirstMatchingStep()
    }

    private fun moveUntilNextZ(current: NodeStep): NodeStep {
        val (node, step) = current
        val directionPosition = (step % directions.length).toInt()

        val cached = toZCache[node]?.get(directionPosition)

        if (cached != null)
            return cached
                .let { (newNode, offset) -> NodeStep(newNode, step + offset) }

        cacheStepsUntilNextZ(node, directionPosition)
        return toZCache[node]?.get(directionPosition)!!
            .let { (newNode, offset) -> NodeStep(newNode, step + offset) }
    }

    private tailrec fun cacheStepsUntilNextZ(
        current: String,
        position: Int,
        previousSteps: MutableList<NodePosition> = mutableListOf()
    ) {
        val cached = toZCache[current]?.get(position)

        if (cached != null) {
            toZCache.addAllPreviousSteps(previousSteps, cached.node, cached.step + 1)
            return
        }

        val nextDestination = getNextDestinationByPosition(directions, mappings, current, position)

        if (nextDestination.endsWith('Z')) {
            previousSteps.add(NodePosition(nextDestination, 1))
            toZCache.addAllPreviousSteps(previousSteps, nextDestination, 1)
            return
        }

        previousSteps.add(NodePosition(current, position))

        cacheStepsUntilNextZ(
            nextDestination,
            (position + 1) % directions.length,
            previousSteps
        )
    }

    private fun MutableMap<String, Array<NodePosition?>>.addAllPreviousSteps(
        previousSteps: List<NodePosition>,
        nextDestination: String,
        offset: Int
    ) {
        val stepsInOrder = previousSteps.reversed()
        stepsInOrder.forEachIndexed { index, (oldNode, oldPosition) ->
            getOrPut(oldNode) { arrayOfNulls(directions.length) }[oldPosition] =
                NodePosition(nextDestination, index + offset)
        }
    }

    private fun Sequence<List<NodeStep>>.extractFirstMatchingStep() =
        drop(1)
            .map { paths -> paths.map { it.step }.distinct() }
            .first { it.count() == 1 }
            .first()
}
