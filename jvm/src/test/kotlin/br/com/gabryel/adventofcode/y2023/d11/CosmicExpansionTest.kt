package br.com.gabryel.adventofcode.y2023.d11

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class CosmicExpansionTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 11, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,374", "input,9681886"])
    fun `Part 1`(file: String, expected: Long) {
        val result = timed("Part 1 - $file") { calculateCosmicExpansionMinimumDistanceOfPairs(files[file]!!, 2) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,10,1030", "input,1000000,791134099634"])
    fun `Part 2`(file: String, scale: Long, expected: Long) {
        val result = timed("Part 2 - $file") { calculateCosmicExpansionMinimumDistanceOfPairs(files[file]!!, scale) }

        assertThat(result, equalTo(expected))
    }
}
