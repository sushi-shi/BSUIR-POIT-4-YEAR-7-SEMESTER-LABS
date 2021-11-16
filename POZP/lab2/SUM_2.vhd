----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:18:52 11/16/2021 
-- Design Name: 
-- Module Name:    SUM_2_STR - Structural 
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

entity SUM_2_STR is
    Port ( X : in  STD_LOGIC_VECTOR (1 downto 0);
           Y : in  STD_LOGIC_VECTOR (1 downto 0);
           S : out  STD_LOGIC_VECTOR (1 downto 0);
			  P : out STD_LOGIC);
end SUM_2_STR;

architecture Structural of SUM_2_STR is
component OR2
    port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           S : out  STD_LOGIC);
end component;
component SUM_1
    port ( X: in  STD_LOGIC;
           Y : in  STD_LOGIC;
           S: out  STD_LOGIC;
			  P: out STD_LOGIC);
end component;

signal tmp1, tmp2, tmp3, tmp4 : STD_LOGIC;

begin
	U1:SUM_1 port map (X(0), Y(0), S(0), tmp1);
	U2:SUM_1 port map (X(1), Y(1), tmp2, tmp3);
	U3:SUM_1 port map (tmp1, tmp2, S(1), tmp4);
	U4:OR2 port map (tmp4, tmp3, P);



end Structural;

