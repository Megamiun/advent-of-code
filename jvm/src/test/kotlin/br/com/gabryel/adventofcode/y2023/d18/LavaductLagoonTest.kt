package br.com.gabryel.adventofcode.y2023.d18

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class LavaductLagoonTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 18, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,62", "input,106459"])
    fun `Part 1`(file: String, expected: Long) {
        val result = timed("Part 1 - $file") { findDefaultLagoonAreaFrom(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,952408144115", "input,63806916814808"])
    fun `Part 2`(file: String, expected: Long) {
        val result = timed("Part 2 - $file") { findColorLagoonAreaFrom(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}