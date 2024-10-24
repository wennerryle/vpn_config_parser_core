import numpy as np
import pandas as pd

from vpn_config_parser import Declare

def flatten_declare_objects(declares: list[Declare]) -> pd.DataFrame:
    """
    Преобразует список объектов Declare в плоский DataFrame.
    
    Args:
        declares: Список объектов Declare
        
    Returns:
        pd.DataFrame: DataFrame с колонками 'name', 'key', 'type', 'value'
    """
    flattened_data = []
    
    def process_declare(declare: Declare, parent_name: str = "") -> None:
        """
        Рекурсивно обрабатывает объект Declare и его вложенные declarations.
        
        Args:
            declare: Объект класса Declare
            parent_name: Имя родительского объекта для построения полного пути
        """
        full_name = f"{parent_name}.{declare.name}" if parent_name else declare.name
        
        # Обрабатываем константы текущего уровня
        for constant in declare.constants:
            flattened_data.append({
                'name': full_name,
                'key': constant.key,
                'type': constant.type,
                'value': constant.value
            })
        
        # Рекурсивно обрабатываем вложенные declarations
        for subdeclare in declare.declarations:
            process_declare(subdeclare, full_name)
    
    # Обрабатываем каждый объект Declare
    for declare in declares:
        process_declare(declare)
    
    # Создаем DataFrame
    df = pd.DataFrame(flattened_data)
    
    # Если данных нет, возвращаем пустой DataFrame с нужными колонками
    if df.empty:
        return pd.DataFrame(columns=['name', 'key', 'type', 'value'])
    
    # Сортируем для лучшей читаемости
    df = df.sort_values(['name', 'key']).reset_index(drop=True)
    
    return df