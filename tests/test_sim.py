import tskit
import tskit_maturin_starter


def test_sim_haploid_wright_fisher_returns_tree_sequence():
    ts = tskit_maturin_starter.sim_haploid_wright_fisher(
        population_size=100,
        num_generations=10,
        random_seed=42,
        simplify_interval=1,
    )
    assert isinstance(ts, tskit.TreeSequence)
    assert ts.num_samples == 100
    assert ts.sequence_length == 1.0
