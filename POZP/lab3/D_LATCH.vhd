----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:22:59 11/19/2021 
-- Design Name: 
-- Module Name:    D_LATCH - Behavioral 
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

entity D_LATCH is
Port ( D : in STD_LOGIC;
		 Q : inout STD_LOGIC;
		 nQ   : inout STD_LOGIC);
end D_LATCH;

architecture Structural of D_LATCH is

component MY_NOT port (
	A: in STD_LOGIC;
	Q: out STD_LOGIC
);
end component;
component MY_NOR port(
	A: in STD_LOGIC;
	B: in STD_LOGIC;
	Q: out STD_LOGIC
);
end component;


signal nor_el1, nor_el2 : STD_LOGIC;
signal R, S: STD_LOGIC;

begin
	S <= D;
	U1 : MY_NOT port map(D, R);
	
	
	U2 : MY_NOR port map(r, nq, nor_el2);
	U3 : MY_NOR port map(s, q, nor_el1);

	Q <= nor_el2;
	nQ <= nor_el1;

end Structural;

