LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
 
-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
USE ieee.numeric_std.ALL;
 
ENTITY tb_mux IS
END tb_mux;
 
ARCHITECTURE behavior OF tb_mux IS 
 
    -- Component Declaration for the Unit Under Test (UUT)
    COMPONENT MUX2
    PORT(
         A : IN  STD_LOGIC;
         B : IN  STD_LOGIC;
         S : IN  STD_LOGIC;
         Z : OUT STD_LOGIC
        );
    END COMPONENT;
    

   --Inputs
   signal A : STD_LOGIC := '0';
   signal B : STD_LOGIC := '0';
   signal S : STD_LOGIC := '0';

   --Outputs
   signal Z_BEH : STD_LOGIC;
	signal Z_STR:  STD_LOGIC;
  
	signal error: STD_LOGIC;
	signal test_vector: STD_LOGIC_VECTOR(2 downto 0);
   -- No clocks detected in port list. Replace <clock> below with 
   -- appropriate port name 
 
	constant period : time := 10 ns;
BEGIN
 
  -- Instantiate the Unit Under Test (UUT)
   uut_beh: entity MUX2 PORT MAP (
          A => A,
          B => B,
          S => S,
          Z => Z_BEH
        );
      
    uut_struct: entity MUX_STRUCT PORT MAP (
          A => A,
          B => B,
          S => S,
          Z => Z_STR
        );

 
  A <= test_vector(0);
  B <= test_vector(1);
  S <= test_vector(2);
 
   -- Stimulus process
  stim_proc: process
  begin
  
   for i in 0 to 7 loop
     test_vector <= STD_LOGIC_VECTOR(to_unsigned(i, test_vector'length)); 
     wait for period;
   end loop;
  
  report "End of simulation" severity failure;
  
  end process;
  
  error <= Z_BEH xor Z_STR;

END;