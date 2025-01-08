package br.com.gabryel.adventofcode.y2023.d25

import kotlin.math.max
import kotlin.math.min

private val EMPTY = IntArray(0)

private val wordFinder = "\\w+".toRegex()

fun getProductsOfSnowverload(lines: List<String>): Int {
    val nodes = lines.flatMap { wordFinder.findAll(it).map { it.value } }
        .distinct()
        .mapIndexed { index, word -> word to index }
        .toMap()

    val connections = lines.flatMap { line ->
        val (left, others) = line.split(": ")
        val nodeLeft = nodes[left]!!
        others.split(" ").map { sortedIndices(nodeLeft, nodes[it]!!) }
    }

    return Context(connections, nodes.size).findProductOfSplitGroups()
}

private class Context(private val edges: List<Pair<Int, Int>>, private val numberOfNodes: Int) {
    private val minDistances = initializeMinDistances()
    private val paths = initializePaths()
    private val maxEdgesToCheck = 300

    fun findProductOfSplitGroups() = updatePaths().firstNotNullOf { searchCuttableCables() }

    private fun searchCuttableCables(): Int? {
        val frequency = sortEdgesByFrequency()
        val lastToCheck = min(maxEdgesToCheck, frequency.size - 1)

        (0..lastToCheck).forEach { first ->
            (first + 1..lastToCheck).forEach { second ->
                (second + 1..lastToCheck).forEach { third ->
                    val reducedEdges = edges - setOf(frequency[first], frequency[second], frequency[third])

                    val newConnectionsMap = (0 until numberOfNodes).associateWith { node ->
                        reducedEdges.filter { it.first == node }.map { it.second } +
                                reducedEdges.filter { it.second == node }.map { it.first }
                    }

                    val visitedNodes = BooleanArray(numberOfNodes).apply {
                        set(0, true)
                    }

                    val toVisit = ArrayList<Int>().apply {
                        add(0)
                    }

                    while (toVisit.isNotEmpty()) {
                        val current = toVisit.removeLast()
                        val nextVisits = newConnectionsMap[current]!!
                            .filterNot { visitedNodes[it] }

                        nextVisits.forEach {
                            visitedNodes[it] = true
                        }

                        toVisit += nextVisits
                    }

                    if (visitedNodes.any { !it })
                        return visitedNodes.count { it } * visitedNodes.count { !it }
                }
            }
        }

        return null
    }

    private fun sortEdgesByFrequency() = paths.flatten().reversed()
        .flatMapIndexed { index, values ->
            values.asSequence().take(index).windowed(2) { (l, r) -> sortedIndices(l, r) }
        }
        .groupingBy { it }.eachCount().entries
        .sortedByDescending { it.value }
        .map { it.key }

    private fun updatePaths() = sequence {
        var changed = true
        while (changed) {
            changed = false

            for (start in 0 until numberOfNodes) {
                for (end in 0 until numberOfNodes) {
                    if (start == end) continue

                    var minDistance = minDistances[start][end]
                    var minIntermediate: Int? = null

                    for (intermediate in 0 until numberOfNodes) {
                        val toIntermediate = minDistances[start][intermediate]
                        val fromIntermediate = minDistances[intermediate][end]

                        if (toIntermediate == Int.MAX_VALUE || fromIntermediate == Int.MAX_VALUE)
                            continue

                        val candidate = toIntermediate + fromIntermediate
                        if (minDistance > candidate) {
                            minDistance = candidate
                            minIntermediate = intermediate
                        }
                    }

                    if (minIntermediate != null) {
                        minDistances[start][end] = minDistance
                        minDistances[end][start] = minDistance
                        paths[start][end] = paths[start][minIntermediate] + paths[minIntermediate][end]
                        paths[end][start] = (paths[start][end].dropLast(1).reversed() + start).toIntArray()
                        changed = true
                    }
                }

                if (start % 200 == 0 && start != 0)
                    yield(true)
            }

            yield(true)
        }
    }

    private fun initializeMinDistances() = Array(numberOfNodes) { IntArray(numberOfNodes) { Int.MAX_VALUE } }.apply {
        for (it in 0 until numberOfNodes) this[it][it] = 0

        edges.forEach { (l, r) ->
            this[l][r] = 1
            this[r][l] = 1
        }
    }

    private fun initializePaths() = Array(numberOfNodes) { Array(numberOfNodes) { EMPTY } }.apply {
        edges.forEach { (l, r) ->
            this[l][r] = intArrayOf(r)
            this[r][l] = intArrayOf(l)
        }
    }
}

private fun sortedIndices(left: Int, right: Int) = min(left, right) to max(left, right)
