package br.com.gabryel.adventofcode.y2023.d10

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class PipeMazeTest {
    private val files = listOf("sample", "sample-3", "sample-7", "sample-8", "input")
        .associateWith { readLines(2023, 10, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,4", "sample-3,8", "input,6890"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getPipeMazeMaximumDistance(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample-7,8", "sample-8,10", "input,453"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { countPipeMazeInternalPositions(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}