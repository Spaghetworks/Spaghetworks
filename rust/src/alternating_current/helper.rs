mod test {
    #[test]
    /// A single 9v battery with a 2 ohm resistor across it. The -ve terminal is on node_a, the +ve terminal is on node_b,
    /// positive current is measured with current flowing from node_b to node_a.
    fn single_resistor() {
        use crate::alternating_current::ac_system::{AcSystem, ConstraintBuilder};
        use nalgebra::Complex;
        use num_traits::{One, Zero};

        let mut system: AcSystem<&str> = AcSystem::new();
        let resistor_impedence = Complex::new(2.0, 0.0);
        let voltage_source_voltage = Complex::new(9.0, 0.0);
        let epsilon = 1e-10;

        let node_a = system.register_vertex("Node A");
        let node_b = system.register_vertex("Node B");
        let resistor_current = system.register_vertex("Resistor current");
        let voltage_source_current = system.register_vertex("Voltage source current");

        let _ground_constraint = ConstraintBuilder::new()
            .set_vertex_constraint(node_a, Complex::one())
            .set_constant_constraint(Complex::zero())
            .finalize(&mut system);

        let _current_invariant_constraint = ConstraintBuilder::new()
            .set_vertex_constraint(resistor_current, Complex::one())
            .set_vertex_constraint(voltage_source_current, -Complex::one())
            .finalize(&mut system);

        let _voltage_source_constraint = ConstraintBuilder::new()
            .set_vertex_constraint(node_a, -Complex::one())
            .set_vertex_constraint(node_b, Complex::one())
            .set_constant_constraint(-voltage_source_voltage)
            .finalize(&mut system);

        let _impedence_constraint = ConstraintBuilder::new()
            .set_vertex_constraint(resistor_current, resistor_impedence)
            .set_vertex_constraint(node_a, Complex::one())
            .set_vertex_constraint(node_b, -Complex::one())
            .finalize(&mut system);

        let measured_a_voltage = system.get_vertex_result(node_a);
        let measured_b_voltage = system.get_vertex_result(node_b);
        let measured_resistor_current = system.get_vertex_result(resistor_current);
        let measured_voltage_source_current = system.get_vertex_result(voltage_source_current);

        system.print_matrix();
        println!(
            "Node A index: {:?}",
            system.vertices.get_vertex_result_index(node_a),
        );
        println!(
            "Node B index: {:?}",
            system.vertices.get_vertex_result_index(node_b),
        );
        println!(
            "Resistor current index: {:?}",
            system.vertices.get_vertex_result_index(resistor_current),
        );
        println!(
            "Battery current index: {:?}",
            system
                .vertices
                .get_vertex_result_index(voltage_source_current),
        );
        println!("Measured A voltage: {:?}", measured_a_voltage);
        println!("Measured B voltage: {:?}", measured_b_voltage);
        println!("Measured resistor current: {:?}", measured_resistor_current);
        println!(
            "Measured battery current: {:?}",
            measured_voltage_source_current
        );

        assert!(
            (measured_a_voltage.expect("Voltage at node A should be defined")
                - Complex::new(0.0, 0.0))
            .norm()
                < epsilon,
            "Voltage at node A should be near 0 + 0j",
        );

        assert!(
            (measured_b_voltage.expect("Voltage at node B should be defined")
                - Complex::new(9.0, 0.0))
            .norm()
                < epsilon,
            "Voltage at node B should be near 9 + 0j",
        );

        assert!(
            (measured_resistor_current.expect("Resistor current should be defined")
                - Complex::new(4.5, 0.0))
            .norm()
                < epsilon,
            "Current through resistor should be near 4.5 + 0j",
        );

        assert!(
            (measured_voltage_source_current.expect("Battery current should be defined")
                - Complex::new(4.5, 0.0))
            .norm()
                < epsilon,
            "Current through battery should be near 4.5 + 0j",
        );
    }
}
