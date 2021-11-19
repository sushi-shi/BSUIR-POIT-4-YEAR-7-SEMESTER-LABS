----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:55:08 11/19/2021 
-- Design Name: 
-- Module Name:    tb_dlatch_async_reset - Behavioral 
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
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity tb_dlatch_async_reset is
end tb_dlatch_async_reset;

architecture Behavioral of tb_dlatch_async_reset is

    -- Component Declaration for the Unit Under Test (UUT)

    COMPONENT DLATCH_ASYNC_RESET
    Port ( D : in  STD_LOGIC;
			  E : in  STD_LOGIC;
			  CLR : in STD_LOGIC;
           Q : out  STD_LOGIC;
           nQ : out  STD_LOGIC);
    END COMPONENT;
    

   --Inputs
   signal D : std_logic := '1';
	signal E : std_logic := '1';
	signal CLR : std_logic := '0';
	
 	--Outputs
   signal Q : std_logic;
   signal nQ : std_logic;

   -- Clock period definitions
   constant Clk_period : time := 10 ns;
 
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut: DLATCH_ASYNC_RESET PORT MAP (
          D => D,
          E => E,
			 CLR => CLR,
          Q => Q,
          nQ => nQ
        );

   -- Stimulus process
   stim_proc: process
   begin		
		CLR <= '0';
		wait for 100 ns;
		
		E <= '0';
		D <= '0';
		wait for 100 ns;
		
		D <= '1';
      wait for 100 ns;	
		
		E <= '1';
      D <= '1';
      wait for 100 ns;
	
		D <= '0';
      wait for 100 ns;	
		
		D <= '1';
      wait for 100 ns;	
		
		CLR <= '1';
		wait for 100 ns;
		 
		D <= '0';
      wait for 100 ns;	
		
		D <= '1';
      wait for 100 ns;
		
		CLR <= '0';
		wait for 100 ns;
		
		D <= '0';
      wait for 100 ns;	
		
		D <= '1';
      wait for 100 ns;
      wait;
   end process;

END;
