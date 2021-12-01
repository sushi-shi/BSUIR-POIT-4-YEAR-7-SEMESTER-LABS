----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    13:55:17 11/20/2021 
-- Design Name: 
-- Module Name:    tb_johnson - Behavioral 
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

entity tb_johnson is
end tb_johnson;

architecture Behavioral of tb_johnson is

    COMPONENT JOHNSON_COUNTER
    port (
		RST : in  std_logic;
      CLK : in  std_logic;
		LS  : in  std_logic;
		Pin : in  std_logic_vector(0 to 7);
		Pout: out std_logic_vector(0 to 7)
	  );
	 END COMPONENT;

   --Inputs
   signal CLK : std_logic := '0';
   signal RST : std_logic := '0';
	signal LS  : std_logic := '0';
	signal Pin : std_logic_vector(0 to 7) := (others => '0');
	
	--Outputs
	signal Pout: std_logic_vector(0 to 7);

	
   constant CLK_period : time := 10 ns;
	
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
	uut1: JOHNSON_COUNTER PORT MAP (
		CLK => CLK,
		RST => RST,
		LS  => LS,
		Pin => Pin,
		Pout => Pout
	);
		  
   Clk_process: process
   begin
		CLK <= not(CLK);
		wait for CLK_period/2;
   end process;
	
   stim_proc: process
   begin		
		wait for CLK_period;
		
		RST <= '0'; wait for CLK_period;
		RST <= '1'; wait for 2 * CLK_period;
		RST <= '0'; wait for CLK_period;
		
		Pin <= "10101010"; wait for CLK_period;
		LS <= '1'; wait for 16 * CLK_period;
		
      wait;
   end process;
END;