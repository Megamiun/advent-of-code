package br.com.gabryel.adventofcode.y2023.d14

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class ParabolicReflectorDishTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 14, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,136", "input,112046"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { calculateStressAfterSingleMovement(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,64", "input,104619"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { calculateStress(files[file]!!, 1000000000) }

        assertThat(result, equalTo(expected))
    }
}