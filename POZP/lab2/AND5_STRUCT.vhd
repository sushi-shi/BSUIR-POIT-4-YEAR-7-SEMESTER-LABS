----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:54:47 11/16/2021 
-- Design Name: 
-- Module Name:    AND5_STRUCT - Behavioral 
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

entity AND5_STRUCT is
    Port ( A : in  STD_LOGIC_VECTOR (4 downto 0);
           S : out  STD_LOGIC);
end AND5_STRUCT;

architecture Structural of AND5_STRUCT is

component AND2
    port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           S : out  STD_LOGIC);
end component;

signal tmp : STD_LOGIC_VECTOR(3 downto 0);

begin
	GEN_0: AND2 port map (A(0), A(1), tmp(0));
	SCH: FOR J in 1 to 3 GENERATE
		GEN_J: AND2 port map (tmp(J-1), A(J+1), tmp(J));
	End GENERATE;
	
	S <= tmp(3);

end Structural;

