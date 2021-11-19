----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:13:29 11/19/2021 
-- Design Name: 
-- Module Name:    DFF_ENABLE - Behavioral 
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

entity DFF_ENABLE is

Port ( D : in STD_LOGIC;
			 CLK : in STD_LOGIC;
			 ENB : in STD_LOGIC;
			 Q   : out STD_LOGIC);
end DFF_ENABLE;
architecture Behavioral of DFF_ENABLE is
signal Q_TMP: STD_LOGIC;
begin
	main: process(D, CLK, ENB)
	begin
		if (rising_edge(CLK)) then
			if ENB = '1' then 
				Q_TMP <= D;
			end if;
		end if;

	end process;

	Q <= Q_TMP;

end Behavioral;

