
-- VHDL Instantiation Created from source file mult.vhd -- 12:08:34 11/02/2021
--
-- Notes: 
-- 1) This instantiation template has been automatically generated using types
-- std_logic and std_logic_vector for the ports of the instantiated module
-- 2) To use this template to instantiate this entity, cut-and-paste and then edit

	COMPONENT mult
	PORT(
		A : IN std_logic_vector(1 downto 0);
		B : IN std_logic_vector(1 downto 0);
		S : IN std_logic;          
		X : OUT std_logic_vector(1 downto 0)
		);
	END COMPONENT;

	Inst_mult: mult PORT MAP(
		A => ,
		B => ,
		S => ,
		X => 
	);


