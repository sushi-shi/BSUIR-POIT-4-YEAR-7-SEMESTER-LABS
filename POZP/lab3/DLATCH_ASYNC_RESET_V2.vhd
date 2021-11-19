----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:37:36 11/19/2021 
-- Design Name: 
-- Module Name:    DLATCH_ASYNC_RESET_V2 - Behavioral 
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

entity DLATCH_ASYNC_RESET_V2 is
	Port ( D : in STD_LOGIC;
			 CLR : in STD_LOGIC;
			 Q   : out STD_LOGIC);
end DLATCH_ASYNC_RESET_V2;

architecture Behavioral of DLATCH_ASYNC_RESET_V2 is
	signal Q_TMP: STD_LOGIC;
begin
	main: process(D, CLR)
	begin
		if (CLR = '1') then
			Q_TMP <= '0';
		else
			Q_TMP <= D;
		end if;
	end process;

	Q <= Q_TMP;

end Behavioral;

