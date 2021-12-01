----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    11:53:20 11/20/2021 
-- Design Name: 
-- Module Name:    REG_ASYNC_STR - Behavioral 
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

entity REG_ASYNC_STR is
	generic (N : integer := 8);
	port (
		DIN : IN STD_LOGIC_VECTOR(N - 1 downto 0);
		E   : IN STD_LOGIC;
		OE  : IN STD_LOGIC;
		DOUT: OUT STD_LOGIC_VECTOR(N - 1 downto 0));
end REG_ASYNC_STR;

architecture Behavioral of REG_ASYNC_STR is
component DELATCH is
    Port ( D : in  STD_LOGIC;
           E : in  STD_LOGIC;
           Q : out  STD_LOGIC);
end component;
    
signal DIN_BUS : STD_LOGIC_VECTOR(N-1 downto 0);
signal DOUT_BUS : STD_LOGIC_VECTOR(N-1 downto 0);

constant ALLZ : std_logic_vector(N-1 downto 0) := (others => 'Z');

begin
	latches: for i in 0 to N-1 generate
		ci: DELATCH port map (
			D => DIN_BUS(i),
			E => E,
			Q => DOUT_BUS(i)
		);
	end generate;
	
	DIN_BUS <= DIN;
	DOUT <= DOUT_BUS when OE = '1' else ALLZ;


end Behavioral;

