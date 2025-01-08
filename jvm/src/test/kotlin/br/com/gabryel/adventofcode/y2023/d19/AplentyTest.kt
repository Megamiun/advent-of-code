package br.com.gabryel.adventofcode.y2023.d19

import br.com.gabryel.adventofcode.util.readGroupsOfLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class AplentyTest {
    private val files = listOf("sample", "input")
        .associateWith { readGroupsOfLines(2023, 19, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,19114", "input,330820"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getSumOfApprovals(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,167409079868000", "input,123972546935551"])
    fun `Part 2`(file: String, expected: Long) {
        val result = timed("Part 2 - $file") { getPossiblePieces(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}