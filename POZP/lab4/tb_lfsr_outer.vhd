----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    19:35:27 11/20/2021 
-- Design Name: 
-- Module Name:    tb_lfsr_outer - Behavioral 
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

entity tb_lfsr_outer is
end tb_lfsr_outer;

architecture Behavioral of tb_lfsr_outer is

    COMPONENT LFSR_OUTER
    port (
		CLK : in std_logic;
		RST : in std_logic;
		Pout: out std_logic_vector(0 to 5)
	 );
	 END COMPONENT;
	 
    COMPONENT LFSR_INNER
    port (
		CLK : in std_logic;
		RST : in std_logic;
		Pout: out std_logic_vector(0 to 5)
	 );
	 END COMPONENT;
	 
	--Inputs
   signal CLK : std_logic := '0';
   signal RST : std_logic := '0';
	
 	--Outputs
   signal Q : std_logic_vector(0 to 5);
	signal Q_INNER : std_logic_vector(0 to 5);
	
   constant Clk_period : time := 20 ns;
	
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut1: LFSR_OUTER PORT MAP (
			 CLK => CLK,
			 RST => RST,
			 Pout => Q
        );
   uu2: LFSR_INNER PORT MAP (
			 CLK => CLK,
			 RST => RST,
			 Pout => Q_INNER
        );		
		  
   Clk_process: process
   begin
		CLK <= not(CLK);
		wait for Clk_period/2;
   end process;
	
   stim_proc: process
   begin		
		wait for CLK_period;
		
		RST <= '1'; wait for 2 * CLK_period;
		RST <= '0'; wait for CLK_period;
				
      wait;
   end process;
END;