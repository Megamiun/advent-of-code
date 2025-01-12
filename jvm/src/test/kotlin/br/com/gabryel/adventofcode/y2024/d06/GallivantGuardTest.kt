package br.com.gabryel.adventofcode.y2024.d06

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class GallivantGuardTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2024, 6, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,41", "input,5305"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getGuardVisitedCount(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,6", "input,2143"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getGuardLoopsAfterObstacle(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}
