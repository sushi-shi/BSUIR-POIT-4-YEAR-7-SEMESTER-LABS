----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    18:26:12 11/19/2021 
-- Design Name: 
-- Module Name:    tb_bistable - Behavioral 
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
--USE ieee.numeric_std.ALL;
 
ENTITY tb_bistable IS
END tb_bistable;
 
ARCHITECTURE behavior OF tb_bistable IS 
 
    -- Component Declaration for the Unit Under Test (UUT)
 
    COMPONENT Bistable
    PORT(
         Q : OUT  std_logic;
         nQ : OUT  std_logic
        );
    END COMPONENT;
    

 	--Outputs
   signal Q : std_logic;
   signal nQ : std_logic;
   -- No clocks detected in port list. Replace <clock> below with 
   -- appropriate port name 
 
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut: Bistable PORT MAP (
          Q => Q,
          nQ => nQ
        );


   -- Stimulus process
   stim_proc: process
   begin		
      -- hold reset state for 100 ns.
      wait for 100 ns;	

      Q <= '1';
		
		wait for 100 ns;
		
		Q <= '0';
		
		wait for 100 ns;
		
		nQ <= '0';
		
		wait for 100 ns;
		
		nQ <= '1';

      wait;
   end process;

END;
