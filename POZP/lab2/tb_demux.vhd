----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:44:12 11/16/2021 
-- Design Name: 
-- Module Name:    tb_demux - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
 
-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
USE ieee.numeric_std.ALL;
 
ENTITY tb_demux1x4v2 IS
END tb_demux1x4v2;
 
ARCHITECTURE behavior OF tb_demux1x4v2 IS 
 
   --Inputs
   signal S : std_logic_vector (1 downto 0) := "00";
   signal X : std_logic := '0';

   --Outputs
   signal Z_BEH : std_logic_vector (3 downto 0);
	signal Z_STR : std_logic_vector (3 downto 0);
  
  signal error: std_logic_vector (3 downto 0);
  signal test_vector: std_logic_vector (2 downto 0);
   -- No clocks detected in port list. Replace <clock> below with 
   -- appropriate port name 
 
 constant period : time := 10 ns;
BEGIN

  -- Instantiate the Unit Under Test (UUT)
  uut_beh: entity work.DEMUX_1x4 PORT MAP (
		X => X,
		S => S,
		Z => Z_BEH
  );
  uut_struct: entity work.DEMUX_1x4_STRUCT PORT MAP (
		X => X,
		S => S,
		Z => Z_STR
  );

 
  X <= test_vector(0);
  S(0)<=test_vector(1);
  S(1)<=test_vector(2);
 
   -- Stimulus process
  stim_proc: process
  begin
  
   for i in 0 to 7 loop
      
        test_vector <= std_logic_vector(to_unsigned(i, test_vector'length)); 
      wait for period;
    end loop;
  
  report "End of simulation" severity failure;
  
  end process;
  
  error<= Z_STR xor Z_BEH;

END;
