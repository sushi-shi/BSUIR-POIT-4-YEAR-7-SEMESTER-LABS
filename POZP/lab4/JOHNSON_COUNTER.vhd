----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    13:30:31 11/20/2021 
-- Design Name: 
-- Module Name:    JOHNSON_COUNTER - Behavioral 
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

entity JOHNSON_COUNTER is
    generic (i: integer := 3);
    port (
		RST : in  std_logic;
      CLK : in  std_logic;
		LS  : in  std_logic;
		Pin : in  std_logic_vector(0 to 2**i - 1);
		Pout: out std_logic_vector(0 to 2**i - 1)
	  );
end JOHNSON_COUNTER;

architecture Behavioral of JOHNSON_COUNTER is

signal sreg: std_logic_vector(0 to 2 ** i - 1);
signal sdat: std_logic_vector(0 to 2 ** i - 1);

begin

  main: process( CLK, RST, sdat )
  begin
    if RST = '1' then
      sreg <= ( others => '0' );
    elsif rising_edge(CLK) then
      sreg <= sdat;
    end if;
  end process;
  
  data: process( LS, Pin, sreg ) 
  begin
	 if LS = '0' then
		sdat <= Pin;
	 else 
		sdat <= not(sreg( 2 ** i - 1)) & sreg ( 0 to 2 ** i - 2 );
	 end if;
  end process;

  Pout <= sdat;
    

end Behavioral;


