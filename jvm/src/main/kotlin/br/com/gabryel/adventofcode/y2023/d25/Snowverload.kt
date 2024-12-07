package br.com.gabryel.adventofcode.y2023.d25

import br.com.gabryel.adventofcode.y2023.logTimed
import br.com.gabryel.adventofcode.readLines
import kotlin.math.max
import kotlin.math.min

private const val EDGES_TO_CHECK_PER_ITERATION = 15

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(2023, 25, file)

        val nodes = lines.flatMap { "\\w+".toRegex().findAll(it).map { it.value } }
            .distinct()
            .mapIndexed { index, word -> word to index }
            .toMap()

        val connections = lines.flatMap {
            val (left, others) = it.split(": ")
            others.split(" ").map { min(nodes[it]!!, nodes[left]!!) to max(nodes[it]!!, nodes[left]!!) }
        }

        println("[Product Of Groups][$file] ${getProductOfGroups(connections, nodes)}")
    }
}

private fun getProductOfGroups(edges: List<Pair<Int, Int>>, nodes: Map<String, Int>) =
    getMostUsedPerIteration(edges, nodes.size)
        .map { findProductOfGroups(it, nodes.size) }
        .filter { it != 0 }
        .first()

private fun findProductOfGroups(edges: List<Pair<Int, Int>>, nodeNumber: Int): Int {
    (0..EDGES_TO_CHECK_PER_ITERATION).forEach { first ->
        (first + 1..EDGES_TO_CHECK_PER_ITERATION).forEach { second ->
            (second + 1..EDGES_TO_CHECK_PER_ITERATION).forEach { third ->
                val toRemove = setOf(first, second, third)
                val newConnections = edges.filterIndexed { index, _ -> index !in toRemove }

                val newConnectionsMap = (0 until nodeNumber).associateWith { node ->
                    newConnections.filter { it.first == node }.map { it.second } +
                            newConnections.filter { it.second == node }.map { it.first }
                }

                val visitedNodes = BooleanArray(nodeNumber).apply {
                    set(0, true)
                }
                val toVisit = ArrayDeque<Int>().apply {
                    add(0)
                }

                while (toVisit.isNotEmpty()) {
                    val current = toVisit.removeFirst()

                    val nextVisits = newConnectionsMap[current]!!
                        .filterNot { visitedNodes[it] }

                    nextVisits.forEach {
                        visitedNodes[it] = true
                    }

                    toVisit.addAll(nextVisits)
                }

                if (visitedNodes.any { !it })
                    return visitedNodes.count { it } * visitedNodes.count { !it }
            }
        }
    }
    return 0
}

private fun getMostUsedPerIteration(edges: List<Pair<Int, Int>>, nodes: Int): Sequence<List<Pair<Int, Int>>> {
    val minDistances = Array(nodes) { IntArray(nodes) { Int.MAX_VALUE } }
    val paths = Array(nodes) { index -> Array(nodes) { intArrayOf() } }

    edges.forEach { (first, second) ->
        minDistances[first][second] = 1
        minDistances[second][first] = 1
        paths[first][second] = intArrayOf(second)
        paths[second][first] = intArrayOf(first)
    }

    (0 until nodes).forEach {
        minDistances[it][it] = 0
    }

    return updateShortestPaths(nodes, minDistances, paths)
        .map { paths.sortEdgesByFrequency() }
        .map { it + (edges - it) }
}

private fun Array<Array<IntArray>>.sortEdgesByFrequency() = flatten()
    .flatMap { it.asSequence().windowed(2) { (l, r) -> min(l, r) to max(l, r) } }
    .groupingBy { it }
    .eachCount()
    .entries
    .sortedByDescending { it.value }
    .map { it.key }

private fun updateShortestPaths(nodes: Int, minDistances: Array<IntArray>, path: Array<Array<IntArray>>) = sequence {
    var iterations = 0
    var changed = true
    while (changed) {
        changed = false
        iterations++
        logTimed("Starting Warshaw Iteration #$iterations")

        (0 until nodes).forEach { start ->
            (0 until nodes).forEach { end ->
                (0 until nodes).forEach { intermediate ->
                    val toIntermediate = minDistances[start][intermediate]
                    val fromIntermediate = minDistances[intermediate][end]

                    if (toIntermediate == Int.MAX_VALUE || fromIntermediate == Int.MAX_VALUE)
                        return@forEach

                    val candidate = toIntermediate + fromIntermediate
                    if (minDistances[start][end] > candidate) {
                        minDistances[start][end] = candidate
                        minDistances[end][start] = candidate
                        path[start][end] = path[start][intermediate] + path[intermediate][end]
                        path[end][start] = (path[start][end].dropLast(1).reversed() + start).toIntArray()
                        changed = true
                    }
                }
            }

            if (start % (nodes / 5) == 0 && start != 0) {
                val done = String.format("%.0f", (100 * start.toDouble() / nodes))
                logTimed("Warshaw Iteration #$iterations $done% finished. Sending signal to check again.")
                yield(iterations)
            }
        }

        logTimed("Warshaw Iteration #$iterations finished. Sending signal to check again.")
        yield(iterations)
    }
}
