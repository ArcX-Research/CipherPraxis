#!/usr/bin/env python3
"""Small independent finite checks for the reference's optimized pseudocode.

These generated symbols test algebra and complete-set identities, not language
recovery or the external research executables. No research corpus is loaded.
"""
import itertools
import unittest
from collections import Counter


def rank(rows, prime):
    rows = [[x % prime for x in row] for row in rows]
    if not rows:
        return 0
    pivot = 0
    for col in range(len(rows[0])):
        found = next((i for i in range(pivot, len(rows)) if rows[i][col]), None)
        if found is None:
            continue
        rows[pivot], rows[found] = rows[found], rows[pivot]
        inverse = pow(rows[pivot][col], -1, prime)
        rows[pivot] = [x * inverse % prime for x in rows[pivot]]
        for i in range(pivot + 1, len(rows)):
            factor = rows[i][col]
            rows[i] = [(x - factor * y) % prime for x, y in zip(rows[i], rows[pivot])]
        pivot += 1
        if pivot == len(rows):
            break
    return pivot


def bm(sequence, prime):
    """Connection polynomial; convert its signs to forward recurrence weights."""
    connection, previous = [1], [1]
    degree, distance, last = 0, 1, 1
    for i, symbol in enumerate(sequence):
        discrepancy = (symbol + sum(connection[j] * sequence[i - j]
                                   for j in range(1, degree + 1))) % prime
        if not discrepancy:
            distance += 1
            continue
        saved = connection[:]
        scale = discrepancy * pow(last, -1, prime) % prime
        connection += [0] * max(0, len(previous) + distance - len(connection))
        for j, value in enumerate(previous):
            connection[j + distance] = (connection[j + distance] - scale * value) % prime
        if 2 * degree <= i:
            degree, previous, last, distance = i + 1 - degree, saved, discrepancy, 1
        else:
            distance += 1
    return [(-connection[j]) % prime for j in range(1, degree + 1)]


class PotentialForest:
    def __init__(self, size, modulus):
        self.parent = list(range(size))
        self.size = [1] * size
        self.weight = [0] * size
        self.modulus = modulus

    def find(self, vertex):
        if self.parent[vertex] == vertex:
            return vertex, 0
        root, weight = self.find(self.parent[vertex])
        self.weight[vertex] = (self.weight[vertex] + weight) % self.modulus
        self.parent[vertex] = root
        return root, self.weight[vertex]

    def join(self, left, right, difference):
        a, x = self.find(left)
        b, y = self.find(right)
        if a == b:
            return (x - y - difference) % self.modulus == 0
        delta = (difference - x + y) % self.modulus
        if self.size[a] > self.size[b]:
            a, b, delta = b, a, -delta
        self.parent[a], self.weight[a] = b, delta % self.modulus
        self.size[b] += self.size[a]
        return True


class ResearchAlgorithmChecks(unittest.TestCase):
    def test_cached_word_bridges_against_complete_paths(self):
        # Short initial/final blocks, full-length blocks, ties and duplicate word origins.
        def relax(table, context, score, histories):
            old = table.get(context)
            if old is None or score > old[0]:
                table[context] = (score, set(histories))
            elif score == old[0]:
                old[1].update(histories)

        for order in (1, 3, 4):
            for tied in (False, True):
                def weight(window):
                    return 0 if tied else 3 - sum((i + 1) * x for i, x in enumerate(window)) ** 2

                def full_score(text):
                    return sum(weight(text[i:i + order]) for i in range(len(text) - order + 1))

                for lengths in ((1, 2, 1), (3, 3, 1), (1, 1, 1)):
                    banks = [list(itertools.product(range(2), repeat=length)) for length in lengths]
                    banks[0].append(banks[0][0])  # same text, different origin
                    scores = {(): (0, {()})}
                    for bank in banks:
                        length = len(bank[0])
                        bridge = scores
                        for _ in range(min(length, order - 1)):
                            next_bridge = {}
                            for context, (value, paths) in bridge.items():
                                for symbol in range(2):
                                    extended = context + (symbol,)
                                    delta = weight(extended) if len(extended) == order else 0
                                    relax(next_bridge, extended[-(order - 1):], value + delta, paths)
                            bridge = next_bridge
                        scores = {}
                        for origin, segment in enumerate(bank):
                            prefix = segment[:min(length, order - 1)]
                            for context, (value, paths) in bridge.items():
                                if prefix and context[-len(prefix):] != prefix:
                                    continue
                                suffix = segment[-(order - 1):] if order > 1 else ()
                                if length < order - 1:
                                    suffix = context
                                relax(scores, suffix, value + full_score(segment),
                                      {path + (origin,) for path in paths})
                    best = max(value for value, _ in scores.values())
                    fast = set().union(*(paths for value, paths in scores.values() if value == best))
                    direct = {}
                    for path in itertools.product(*(range(len(bank)) for bank in banks)):
                        text = tuple(x for bank, origin in zip(banks, path) for x in bank[origin])
                        direct[path] = full_score(text)
                    expected = max(direct.values())
                    self.assertEqual(best, expected)
                    self.assertEqual(fast, {path for path, value in direct.items() if value == expected})

    def test_circulant_example_and_composite_invertibility(self):
        h, inverse = [(1, 2), (2, 1)], [(17, 18), (18, 17)]
        self.assertEqual([sum(a * b for a, b in zip(row, (0, 1))) % 26 for row in h], [2, 1])
        self.assertEqual([sum(a * b for a, b in zip(row, (2, 1))) % 26 for row in inverse], [0, 1])
        for values in itertools.product(range(6), repeat=4):
            rows = [values[:2], values[2:]]
            determinant = (values[0] * values[3] - values[1] * values[2]) % 6
            self.assertEqual(determinant in (1, 5), rank(rows, 2) == rank(rows, 3) == 2)

    def test_hill_row_filter_against_complete_matrix_rank(self):
        others = [(1, 2, 3), (0, 1, 1)]
        normals = {}
        for prime in (2, 3):
            normals[prime] = next(v for v in itertools.product(range(prime), repeat=3)
                                  if any(v) and all(sum(a * b for a, b in zip(r, v)) % prime == 0
                                                    for r in others))
        for row in itertools.product(range(6), repeat=3):
            fast = all(sum(a * b for a, b in zip(row, normals[p])) % p for p in (2, 3))
            direct = all(rank(others + [row], p) == 3 for p in (2, 3))
            self.assertEqual(fast, direct)

    def test_potential_forest_complete_observable_set(self):
        edges = [(0, 2, 2), (1, 3, 4), (0, 4, 1)]
        forest = PotentialForest(5, 6)
        for edge in edges:
            self.assertTrue(forest.join(*edge))
        self.assertFalse(forest.join(0, 2, 3))
        roots = sorted({forest.find(v)[0] for v in range(5)})
        fast = set()
        for offsets in itertools.product(range(6), repeat=len(roots) - 1):
            values = dict(zip(roots, (0,) + offsets))
            key = [(forest.find(v)[1] + values[forest.find(v)[0]]) % 6 for v in range(5)]
            fast.add(tuple((key[i % 2] - key[2 + i % 3]) % 6 for i in range(6)))
        direct = set()
        for tail in itertools.product(range(6), repeat=4):
            key = (0,) + tail
            if all((key[a] - key[b]) % 6 == d for a, b, d in edges):
                direct.add(tuple((key[i % 2] - key[2 + i % 3]) % 6 for i in range(6)))
        self.assertEqual(fast, direct)

    def test_gray_code_deltas_include_zero_and_overlapping_windows(self):
        cipher = [0, 1, 0, 2, 1, 3, 0, 3, 2]
        labels = [0] * 4
        def score(bits):
            return sum(7 - (4 * bits[i] + 2 * bits[i + 1] + bits[i + 2]) ** 2
                       for i in range(len(bits) - 2))
        plain, seen = [0] * len(cipher), {0}
        total, mask = score(plain), 0
        best = total
        for counter in range(1, 16):
            bit = (counter & -counter).bit_length() - 1
            positions = [i for i, x in enumerate(cipher) if x == bit]
            starts = {s for i in positions for s in range(max(0, i - 2), min(i, len(cipher) - 3) + 1)}
            before = sum(score(plain[s:s + 3]) for s in starts)
            labels[bit] ^= 1
            for i in positions:
                plain[i] = labels[bit]
            after = sum(score(plain[s:s + 3]) for s in starts)
            total += after - before
            self.assertEqual(total, score(plain))
            mask ^= 1 << bit
            seen.add(mask)
            best = max(best, total)
        self.assertEqual(seen, set(range(16)))
        self.assertEqual(best, score([0] * len(cipher)))

    def test_affine_order_against_exhaustive_composite_recurrences(self):
        # Every length-four sequence over Z6, including the empty-difference case.
        for length in (1, 4):
            for seq in itertools.product(range(6), repeat=length):
                differences = [(b - a) % 6 for a, b in zip(seq, seq[1:])]
                a2, a3 = bm([x % 2 for x in differences], 2), bm([x % 3 for x in differences], 3)
                order = max(len(a2), len(a3))
                a2 += [0] * (order - len(a2))
                a3 += [0] * (order - len(a3))
                coefficients = [(3 * a + 4 * b) % 6 for a, b in zip(a2, a3)]
                constant = (seq[order] - sum(a * seq[order - j - 1] for j, a in enumerate(coefficients))) % 6
                self.assertTrue(all((constant + sum(a * seq[i - j - 1] for j, a in enumerate(coefficients))) % 6 == seq[i]
                                    for i in range(order, length)))
                for smaller in range(order):
                    for trial in itertools.product(range(6), repeat=smaller):
                        residuals = {(seq[i] - sum(a * seq[i - j - 1] for j, a in enumerate(trial))) % 6
                                     for i in range(smaller, length)}
                        self.assertNotEqual(len(residuals), 1)

    def test_triple_curvature_factors_against_direct_peeling(self):
        left, right, modulus = [0, 1, 1, 2, 0, 1], [2, 2, 0, 1, 0, 2], 6
        fast = Counter()
        for j in range(len(left) - 2):
            a = left[j:j + 3]
            for k in range(len(right) - 2):
                b = right[k:k + 3]
                if (a[2] - 2 * a[1] + a[0] - b[2] + 2 * b[1] - b[0]) % modulus:
                    continue
                difference = b[1] - b[0] - a[1] + a[0]
                for u in range(modulus):
                    v = (u + difference) % modulus
                    delta = (b[0] - k * v - a[0] + j * u) % modulus
                    fast[u, v, delta] += 1
        for u, v, delta in itertools.product(range(modulus), repeat=3):
            a = [(x - i * u) % modulus for i, x in enumerate(left)]
            b = [(x - i * v - delta) % modulus for i, x in enumerate(right)]
            direct = sum(a[j:j + 3] == b[k:k + 3]
                         for j in range(len(a) - 2) for k in range(len(b) - 2))
            self.assertEqual(fast[u, v, delta], direct)

    def test_projective_pencil_count_keeps_repeated_members(self):
        prime, dimension = 3, 3
        points = [x for x in itertools.product(range(prime), repeat=dimension)
                  if any(x) and next(v for v in x if v) == 1]
        forms = [lambda x: 0, lambda x: x[0] * x[1], lambda x: x[0] ** 2,
                 lambda x: x[1] * x[2] - x[0] ** 2]
        for first, second in itertools.product(forms, repeat=2):
            counts = [sum((first(x) + t * second(x)) % prime == 0 for x in points)
                      for t in range(prime)]
            counts.append(sum(second(x) % prime == 0 for x in points))
            self.assertEqual((sum(counts) - len(points)) // prime,
                             sum(first(x) % prime == second(x) % prime == 0 for x in points))
            self.assertEqual((sum(counts) - len(points)) % prime, 0)

    def test_observed_completion_and_inverse_query_fibers(self):
        # Physical zero is unobserved: regauging must not divide the fiber by m.
        observed = {1: 0, 2: 3, 4: 1}
        missing_symbols = [0, 3, 5]
        missing_images = [2, 4, 5]
        full, readings = set(), Counter()
        for values in itertools.permutations(missing_images):
            mapping = observed | dict(zip(missing_symbols, values))
            normalized = tuple((mapping[i] - mapping[0]) % 6 for i in range(6))
            full.add(normalized)
            inverse = {v: k for k, v in mapping.items()}
            readings[inverse[2], inverse[4], inverse[2]] += 1
        self.assertEqual(len(full), 6)
        self.assertEqual(len(readings), 6)
        self.assertTrue(all(a == c and a != b for a, b, c in readings))


if __name__ == '__main__':
    unittest.main(verbosity=2)
