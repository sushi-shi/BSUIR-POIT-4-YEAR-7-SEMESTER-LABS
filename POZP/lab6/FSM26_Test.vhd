--------------------------------------------------------------------------------
-- Company: 
-- Engineer:
--
-- Create Date:   12:31:23 11/21/2021
-- Design Name:   
-- Module Name:   F:/XilinxProjects/Lab5/FSM26_Test.vhd
-- Project Name:  Lab5
-- Target Device:  
-- Tool versions:  
-- Description:   
-- 
-- VHDL Test Bench Created by ISE for module: FSM26
-- 
-- Dependencies:
-- 
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
--
-- Notes: 
-- This testbench has been automatically generated using types std_logic and
-- std_logic_vector for the ports of the unit under test.  Xilinx recommends
-- that these types always be used for the top-level I/O of a design in order
-- to guarantee that the testbench will bind correctly to the post-implementation 
-- simulation model.
--------------------------------------------------------------------------------
LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
 
-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--USE ieee.numeric_std.ALL;
 
ENTITY FSM26_Test IS
END FSM26_Test;
 
ARCHITECTURE behavior OF FSM26_Test IS 
 
    -- Component Declaration for the Unit Under Test (UUT)
 
    COMPONENT FSM26
    PORT(
         Clk : IN  std_logic;
         RST : IN  std_logic;
         IP : IN  std_logic_vector(3 downto 0);
         DataOut : OUT  std_logic_vector(1 downto 0)
        );
    END COMPONENT;
    

   --Inputs
   signal Clk : std_logic := '0';
   signal RST : std_logic := '0';
   signal IP : std_logic_vector(3 downto 0) := (others => '0');

 	--Outputs
   signal DataOut : std_logic_vector(1 downto 0);

   -- Clock period definitions
   constant Clk_period : time := 10 ns;
 
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut: FSM26 PORT MAP (
          Clk => Clk,
          RST => RST,
          IP => IP,
          DataOut => DataOut
        );

   -- Clock process definitions
   Clk_process :process
   begin
		Clk <= '0';
		wait for Clk_period/2;
		Clk <= '1';
		wait for Clk_period/2;
   end process;
 

   -- Stimulus process
   stim_proc: process
   begin		
      -- hold reset state for 100 ns.
      wait for 100 ns;	
      RST <= '1';
      wait for Clk_period;
      RST <= '0';
      wait for Clk_period;

      IP <= "0000";
      wait for Clk_period * 2;
      
      IP <= "0001";
      wait for Clk_period * 2;
      
      RST <= '1';
      wait for Clk_period;
      RST <= '0';
      wait for Clk_period;
      
      
      IP <= "0000";
      wait for Clk_period * 2;
      
      IP <= "0100";
      wait for Clk_period * 2;
      
      RST <= '1';
      wait for Clk_period;
      RST <= '0';
      wait for Clk_period;
      
      
      IP <= "0000";
      wait for Clk_period * 2;

      IP <= "0010";
      wait for Clk_period * 2;

      IP <= "0011";
      wait for Clk_period * 2;
      
      IP <= "0010";
      wait for Clk_period * 2;

      IP <= "0011";
      wait for Clk_period * 2;
      wait;
   end process;

END;
