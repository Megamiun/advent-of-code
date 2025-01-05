package br.com.gabryel.adventofcode.y2023.d13

import br.com.gabryel.adventofcode.util.readGroupsOfLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class PointOfIncidenceTest {
    private val files = listOf("sample", "input")
        .associateWith { readGroupsOfLines(2023, 13, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,405", "input,29213"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { sumOfIncidencePoints(files[file]!!, 0) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,400", "input,37453"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { sumOfIncidencePoints(files[file]!!, 1) }

        assertThat(result, equalTo(expected))
    }
}