----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:38:20 11/19/2021 
-- Design Name: 
-- Module Name:    D_LATCH_ENABLE - Behavioral 
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

entity D_LATCH_ENABLE is
    Port ( D : in  STD_LOGIC;
			  E : in  STD_LOGIC;
           Q : inout  STD_LOGIC;
           nQ : inout  STD_LOGIC);
end D_LATCH_ENABLE;

architecture Behavioral of D_LATCH_ENABLE is
component RS_LATCH port (
	S : in STD_LOGIC;
	R : in STD_LOGIC;
	Q : inout STD_LOGIC;
	nQ : inout STD_LOGIC);
end component;

component MY_AND port (
	A : in STD_LOGIC;
	B : in STD_LOGIC;
	Q : out STD_LOGIC);
end component;

component MY_NOT port (
	A : in STD_LOGIC;
	Q : out STD_LOGIC);
end component;

	signal R, S, nD : STD_LOGIC;
	
begin
	U1 : MY_NOT port map(D, nD);
	U2 : MY_AND port map(D, E, S); -- U
	U3 : MY_AND port map(nD, E, R);
	U4 : RS_LATCH port map (S, R, Q, nQ);


end Behavioral;

