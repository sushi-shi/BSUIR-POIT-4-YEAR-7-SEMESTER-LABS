----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    01:43:30 11/20/2021 
-- Design Name: 
-- Module Name:    DEFF - Behavioral 
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

entity DEFF is
    Port ( D : in  STD_LOGIC;
           E : in  STD_LOGIC;
           CLK : in  STD_LOGIC;
           Q : out  STD_LOGIC);
end DEFF;

architecture Behavioral of DEFF is
	signal QBuf : STD_LOGIC;
begin
	main: Process(D, E, CLK)
	begin
		if E = '1' and rising_edge(CLK) then
			QBuf <= D;
		end if;
	end process;
	
	Q <= QBuf;
	
end Behavioral;

